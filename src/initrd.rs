use crate::elf;
use crate::error::InitrdLoadError;
use core::mem::MaybeUninit;
use efi::{CStr16, File};

fn check_placement(mmap: &efi::MemoryMap, base: usize, size: usize) -> bool {
    for page in (base & !0xFFF..(base + size + 0xFFF) & !0xFFF).step_by(0x1000) {
        if !mmap.is_usable_now(page) {
            return false;
        }
    }
    true
}

fn do_load(file: &mut File, base: usize, size: usize) -> Result<(), InitrdLoadError> {
    file.read(unsafe { core::slice::from_raw_parts_mut(base as *mut u8, size) })
        .map_err(InitrdLoadError::IOError)?;
    Ok(())
}

pub fn load_somewhere(
    root: &mut File,
    filename: &CStr16,
    mmap: &efi::MemoryMap,
    obj: &elf::Object,
) -> Result<(usize, usize), InitrdLoadError> {
    let mut statbuf: [u8; 1024] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut file = root
        .open(filename, efi::proto::fp::OPEN_MODE_READ, 0)
        .map_err(InitrdLoadError::IOError)?;
    let stat = file.stat(&mut statbuf).map_err(InitrdLoadError::IOError)?;
    let size = stat.file_size as usize;

    // 1. Try loading right below the kernel
    if obj.start >= size {
        let start = (obj.start - size) & !0xFFF;

        if check_placement(mmap, start, size) {
            println!("Loading initrd below the kernel at 0x{:016x}", start);
            do_load(&mut file, start, size)?;
            return Ok((start, size));
        }
    }

    // 2. Any location above the kernel
    for start in ((obj.end + 0x3FFF) & !0xFFF..0x100000000).step_by(0x1000) {
        if check_placement(mmap, start, size) {
            println!("Loading initrd at 0x{:016x}", start);
            do_load(&mut file, start, size)?;
            return Ok((start, size));
        }
    }

    Err(InitrdLoadError::NoSpace)
}
