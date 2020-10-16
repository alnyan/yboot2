use crate::error::ImageLoadError;
use core::mem::{size_of, MaybeUninit};
use efi::{CStr16, File, MemoryMap};
use yboot2_proto::{LoadProtocol, Magic};

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
    ident: [u8; 16],
    _type: Half,
    machine: Half,
    version: Word,
    entry: Addr,
    phoff: Off,
    shoff: Off,
    flags: Word,
    ehsize: Half,
    phentsize: Half,
    phnum: Half,
    shentsize: Half,
    shnum: Half,
    shstrndx: Half,
}

#[repr(C)]
struct Shdr {
    name: Word,
    _type: Word,
    flags: XWord,
    addr: Addr,
    offset: Off,
    size: XWord,
    link: Word,
    info: Word,
    addralign: XWord,
    entsize: XWord,
}

#[repr(C)]
struct Phdr {
    _type: Word,
    flags: Word,
    offset: Off,
    vaddr: Addr,
    paddr: Addr,
    filesz: XWord,
    memsz: XWord,
    align: XWord,
}

pub struct Object {
    file: File,
    ehdr: Ehdr,

    pub start: usize,
    pub end: usize,
}

unsafe fn any_as_u8_slice<T: Sized>(p: &mut T) -> &mut [u8] {
    ::core::slice::from_raw_parts_mut((p as *mut T) as *mut u8, ::core::mem::size_of::<T>())
}

impl Object {
    // Reason: EFI autism
    pub fn open(root: &mut File, path: &CStr16) -> Result<Object, ImageLoadError> {
        let mut obj = Object {
            file: root
                .open(path, efi::proto::fp::OPEN_MODE_READ, 0)
                .map_err(ImageLoadError::IOError)?,
            ehdr: unsafe { MaybeUninit::uninit().assume_init() },
            start: 0xFFFFFFFFFFFFFFFF,
            end: 0,
        };

        // Load header
        obj.file.seek(0).map_err(ImageLoadError::IOError)?;
        obj.file
            .read(unsafe { any_as_u8_slice(&mut obj.ehdr) })
            .map_err(ImageLoadError::IOError)?;

        // Validate that this is ELF
        if &obj.ehdr.ident[0..4] != [0x7F, b'E', b'L', b'F'] {
            return Err(ImageLoadError::BadMagic);
        }

        // Validate that bitness matches requested
        if obj.ehdr.ident[4] != 2 {
            return Err(ImageLoadError::BadTarget);
        }

        Ok(obj)
    }

    fn read_phdr(&mut self, phdr: &mut Phdr, index: usize) -> Result<(), ImageLoadError> {
        let off = self.ehdr.phoff + self.ehdr.phentsize as u64 * index as u64;
        self.file.seek(off).map_err(ImageLoadError::IOError)?;
        if self
            .file
            .read(unsafe { any_as_u8_slice(phdr) })
            .map_err(ImageLoadError::IOError)?
            != size_of::<Phdr>()
        {
            Err(ImageLoadError::IOError(efi::Status::Err))
        } else {
            Ok(())
        }
    }
    fn read_shdr(&mut self, shdr: &mut Shdr, index: usize) -> Result<(), ImageLoadError> {
        let off = self.ehdr.shoff + self.ehdr.shentsize as u64 * index as u64;
        self.file.seek(off).map_err(ImageLoadError::IOError)?;
        if self
            .file
            .read(unsafe { any_as_u8_slice(shdr) })
            .map_err(ImageLoadError::IOError)?
            != size_of::<Shdr>()
        {
            Err(ImageLoadError::IOError(efi::Status::Err))
        } else {
            Ok(())
        }
    }

    // Called after load()
    pub fn locate_protocol_data<T: Magic + LoadProtocol>(
        &mut self,
    ) -> Result<&'static mut T, ImageLoadError> {
        let mut shdr = unsafe { MaybeUninit::<Shdr>::uninit().assume_init() };

        for i in 0..self.ehdr.shnum {
            self.read_shdr(&mut shdr, i as usize)?;

            if shdr._type == SHT_PROGBITS
                && (shdr.flags & (SHF_ALLOC | SHF_WRITE)) == SHF_ALLOC | SHF_WRITE
            {
                if shdr.size as usize >= size_of::<T>() {
                    // Make a physical address
                    let ptr = shdr.addr - 0xFFFFFF0000000000;
                    let magic: &[u8] = unsafe { core::slice::from_raw_parts(ptr as *const _, 8) };
                    if magic == T::KERNEL_MAGIC {
                        return Ok(unsafe { &mut *(ptr as *mut _) });
                    }
                }
            }
        }

        Err(ImageLoadError::NoProtocol)
    }

    pub fn load(&mut self, mmap: &MemoryMap) -> Result<usize, ImageLoadError> {
        extern "C" {
            fn memset(block: *mut u8, value: i32, count: usize) -> *mut u8;
        }

        let mut phdr = unsafe { MaybeUninit::<Phdr>::uninit().assume_init() };

        // 1. Check that all pages in load segments are usable
        //    Also find out kernel's lowest and highest physical addresses
        for i in 0..self.ehdr.phnum {
            self.read_phdr(&mut phdr, i as usize)?;

            if phdr._type == PT_LOAD {
                if phdr.paddr + phdr.memsz >= 0x100000000 {
                    return Err(ImageLoadError::BadAddress(
                        phdr.paddr + phdr.memsz,
                        0,
                        0x100000000,
                    ));
                }

                let start = phdr.paddr & !0xFFF;
                let end = (phdr.paddr + phdr.memsz as u64 + 0xFFF) & !0xFFF;

                if (start as usize) < self.start {
                    self.start = start as usize;
                }
                if (end as usize) > self.end {
                    self.end = end as usize;
                }

                for addr in (start..end).step_by(0x1000) {
                    if !mmap.is_usable_now(addr as usize) {
                        return Err(ImageLoadError::BadSegment(start, end, addr));
                    }
                }
            }
        }

        // 2. Load segments
        for i in 0..self.ehdr.phnum {
            self.read_phdr(&mut phdr, i as usize)?;

            if phdr._type == PT_LOAD {
                // Load what's provided in ELF
                if phdr.filesz > 0 {
                    let mut data = unsafe {
                        core::slice::from_raw_parts_mut(phdr.paddr as *mut u8, phdr.filesz as usize)
                    };

                    self.file.seek(phdr.offset).map_err(ImageLoadError::IOError)?;
                    self.file.read(&mut data).map_err(ImageLoadError::IOError)?;
                }

                // Zero the rest
                if phdr.memsz > phdr.filesz {
                    unsafe {
                        memset(
                            (phdr.paddr as usize + phdr.filesz as usize) as *mut u8,
                            0,
                            (phdr.memsz - phdr.filesz) as usize,
                        );
                    }
                }
            }
        }

        Ok(self.ehdr.entry as usize)
    }
}
