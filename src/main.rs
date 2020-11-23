#![feature(asm, const_fn, llvm_asm)]
#![no_main]
#![no_std]

extern crate char16_literal;
extern crate core_rt;
extern crate efi;
extern crate yboot2_proto;
pub(crate) use char16_literal::cstr16;

use core::convert::TryInto;
use efi::{
    image_handle, system_table, CStr16, ConfigurationTableEntry, ImageHandle, Status, SystemTable,
};
use yboot2_proto::{LoadProtocol, MemoryMapInfo, ProtoV1};

#[macro_use]
mod println;
mod elf;
mod error;
mod initrd;
mod mem;
mod video;

use error::BootError;

fn set_efi_mmap<T: LoadProtocol>(data: &mut T, mmap: &efi::MemoryMap) -> Result<(), BootError> {
    match data.set_mmap(&MemoryMapInfo {
        address: mmap.storage_ref.as_ptr() as u64,
        entsize: mmap.descriptor_size.try_into().unwrap(),
        size: mmap.size.try_into().unwrap(),
    }) {
        Err(_) => Err(BootError::MemoryMapError(efi::Status::Err)),
        Ok(()) => Ok(()),
    }
}

fn main() -> Result<(), BootError> {
    let mut desc_array = [0u8; 16384];
    let mut mmap = efi::MemoryMap::new(&mut desc_array);
    let bs = &system_table().boot_services;

    bs.get_memory_map(&mut mmap)
        .map_err(BootError::MemoryMapError)?;

    let rsdp = system_table()
        .config_iter()
        .find(|x| matches!(x, ConfigurationTableEntry::Acpi10Table(_)))
        .map(|x| match x {
            ConfigurationTableEntry::Acpi10Table(ptr) => ptr,
            _ => panic!(),
        });

    let mut root = image_handle()
        .get_boot_path()
        .map_err(BootError::FileError)?
        .open_partition()
        .map_err(BootError::FileError)?;

    // Load kernel
    let mut obj = elf::Object::open(&mut root, CStr16::from_literal(cstr16!(r"\kernel.elf")))?;
    let entry = obj.load(&mmap)?;
    let data = obj.locate_protocol_data::<ProtoV1>()?;

    if (data.get_flags() & yboot2_proto::FLAG_INITRD) != 0 {
        // Load initrd
        let (initrd_base, initrd_size) = initrd::load_somewhere(
            &mut root,
            CStr16::from_literal(cstr16!(r"\initrd.img")),
            &mmap,
            &obj,
        )?;

        // Set video mode
        data.set_initrd(initrd_base, initrd_size);
    } else {
        data.set_initrd(0, 0);
    }

    data.set_acpi_rsdp(rsdp.unwrap_or(core::ptr::null_mut()) as usize);
    data.set_loader_magic();

    if (data.get_flags() & yboot2_proto::FLAG_VIDEO) != 0 {
        video::set_mode(bs, data)?;
    }

    // Get the new memory map and terminate boot services
    bs.get_memory_map(&mut mmap).map_err(BootError::MemoryMapError)?;
    bs.exit_boot_services(mmap.key).map_err(BootError::TerminateServicesError)?;
    set_efi_mmap(data, &mmap)?;

    // Setup upper virtual mapping if requested
    let real_entry: usize;
    if (data.get_flags() & yboot2_proto::FLAG_UPPER) != 0 {
        mem::setup_upper();

        real_entry = entry;
    } else {
        real_entry = if entry >= 0xFFFFFF0000000000 {
            entry - 0xFFFFFF0000000000
        } else {
            entry
        };
    }
    unsafe {
        llvm_asm!("xor %rbp, %rbp; jmp *$0"::"{di}"(real_entry));
    }
    loop {}
}

#[no_mangle]
extern "C" fn efi_main(ih: *mut ImageHandle, st: *mut SystemTable) -> u64 {
    efi::init(ih, st);
    let res = &main();
    // Don't return immediately on failure
    if let Err(err) = res {
        let bs = &system_table().boot_services;
        println!("yboot2 error: {}", err);
        // Delay for 5s so error message can be read
        bs.stall(5000000);
    }

    efi::Termination::to_efi(&res.as_ref().map_err(efi::Status::from))
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    // TODO: check if BS are available
    println!("Panic: {}!", panic);
    system_table().boot_services.exit(Status::Err);
}
