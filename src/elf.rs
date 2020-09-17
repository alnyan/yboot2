use efi::{File, Status, CStr16, MemoryMap};
use core::mem::{MaybeUninit, size_of};
use crate::proto::LoadProtocol;

type Off = u64;
type Addr = u64;
type Half = u16;
type Word = u32;
type XWord = u64;

const PT_LOAD: Word = 1;

const SHT_PROGBITS: Word = 1;

const SHF_WRITE: XWord = 1 << 0;
const SHF_ALLOC: XWord = 1 << 1;

#[repr(C)]
struct Ehdr {
    ident:      [u8; 16],
    _type:      Half,
    machine:    Half,
    version:    Word,
    entry:      Addr,
    phoff:      Off,
    shoff:      Off,
    flags:      Word,
    ehsize:     Half,
    phentsize:  Half,
    phnum:      Half,
    shentsize:  Half,
    shnum:      Half,
    shstrndx:   Half
}

#[repr(C)]
struct Shdr {
    name:       Word,
    _type:      Word,
    flags:      XWord,
    addr:       Addr,
    offset:     Off,
    size:       XWord,
    link:       Word,
    info:       Word,
    addralign:  XWord,
    entsize:    XWord
}

#[repr(C)]
struct Phdr {
    _type:      Word,
    offset:     Off,
    vaddr:      Addr,
    paddr:      Addr,
    filesz:     Word,
    memsz:      Word,
    flags:      Word,
    align:      Word
}

pub struct Object {
    file:   File,
    ehdr:   Ehdr,
}

unsafe fn any_as_u8_slice<T: Sized>(p: &mut T) -> &mut [u8] {
    ::core::slice::from_raw_parts_mut(
        (p as *mut T) as *mut u8,
        ::core::mem::size_of::<T>(),
    )
}

impl Object {
    // Reason: EFI autism
    pub fn open(root: &mut File, path: &CStr16) -> Result<Object, Status> {
        let mut obj = Object {
            file: root.open(path, efi::proto::fp::OPEN_MODE_READ, 0)?,
            ehdr: unsafe { MaybeUninit::uninit().assume_init() },
        };

        // Load header
        obj.file.seek(0)?;
        obj.file.read(unsafe {any_as_u8_slice(&mut obj.ehdr)})?;

        // Validate that this is ELF
        if &obj.ehdr.ident[0 .. 4] != [0x7F, b'E', b'L', b'F'] {
            return Err(Status::InvalidParameter);
        }

        // Validate that bitness matches requested
        if obj.ehdr.ident[4] != 2 {
            return Err(Status::InvalidParameter);
        }

        Ok(obj)
    }

    fn read_phdr(&mut self, phdr: &mut Phdr, index: usize) -> Result<(), Status> {
        let off = self.ehdr.phoff + self.ehdr.phentsize as u64 * index as u64;
        self.file.seek(off)?;
        if self.file.read(unsafe { any_as_u8_slice(phdr) })? != size_of::<Phdr>() {
            Err(Status::Err)
        } else {
            Ok(())
        }
    }
    fn read_shdr(&mut self, shdr: &mut Shdr, index: usize) -> Result<(), Status> {
        let off = self.ehdr.shoff + self.ehdr.shentsize as u64 * index as u64;
        self.file.seek(off)?;
        if self.file.read(unsafe { any_as_u8_slice(shdr) })? != size_of::<Shdr>() {
            Err(Status::Err)
        } else {
            Ok(())
        }
    }

    // Called after load()
    pub fn locate_protocol_data<T: LoadProtocol>(&mut self)
        -> Result<&'static mut T, Status>
    {
        let mut shdr = unsafe { MaybeUninit::<Shdr>::uninit().assume_init() };

        for i in 0 .. self.ehdr.shnum {
            self.read_shdr(&mut shdr, i as usize)?;

            if shdr._type == SHT_PROGBITS &&
               (shdr.flags & (SHF_ALLOC | SHF_WRITE)) == SHF_ALLOC | SHF_WRITE {
                if shdr.size as usize >= size_of::<T>() {
                    // Make a physical address
                    let ptr = shdr.addr - 0xFFFFFF0000000000;
                    let magic: &[u8] = unsafe { core::slice::from_raw_parts(ptr as *const _, 8)};
                    if magic == T::KERNEL_MAGIC {
                        return Ok(unsafe { &mut *(ptr as *mut _) })
                    }
                }
            }
        }

        Err(Status::InvalidParameter)
    }

    pub fn load(&mut self, mmap: &MemoryMap) -> Result<usize, Status> {
        extern {
            fn memset(block: *mut u8, value: i32, count: usize) -> *mut u8;
        }

        let mut phdr = unsafe { MaybeUninit::<Phdr>::uninit().assume_init() };

        // 1. Check that all pages in load segments are usable
        for i in 0 .. self.ehdr.phnum {
            self.read_phdr(&mut phdr, i as usize)?;

            if phdr._type == PT_LOAD {
                let start = phdr.paddr & !0xFFF;
                let end = (phdr.paddr + phdr.memsz as u64 + 0xFFF) & !0xFFF;

                for addr in (start .. end).step_by(0x1000) {
                    if !mmap.is_usable_now(addr as usize) {
                        return Err(Status::InvalidParameter);
                    }
                }
            }
        }

        // 2. Load segments
        for i in 0 .. self.ehdr.phnum {
            self.read_phdr(&mut phdr, i as usize)?;

            if phdr._type == PT_LOAD {
                // Load what's provided in ELF
                if phdr.filesz > 0 {
                    let mut data = unsafe {core::slice::from_raw_parts_mut(
                        phdr.paddr as *mut u8,
                        phdr.filesz as usize,
                    )};

                    self.file.seek(phdr.offset)?;
                    self.file.read(&mut data)?;
                }

                // Zero the rest
                if phdr.memsz > phdr.filesz {
                    unsafe {
                        memset((phdr.paddr as usize + phdr.filesz as usize) as *mut u8,
                               0,
                               (phdr.memsz - phdr.filesz) as usize);
                    }
                }
            }
        }

        Ok(self.ehdr.entry as usize - 0xFFFFFF0000000000)
    }
}
