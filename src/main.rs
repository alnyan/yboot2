#![feature(asm)]
#![no_main]
#![no_std]

extern crate efi;
extern crate core_rt;

use efi::{Status, Handle, SystemTable, system_table};

#[macro_use]
mod println;

fn main() -> Status {
    let mut desc_array = [0u8; 4096];
    let mut mmap = efi::MemoryMap::new(&mut desc_array);
    let bs = &system_table().boot_services;
    let rs = &system_table().runtime_services;

    if bs.get_memory_map(&mut mmap) != Status::Success {
        println!("Failed to get memory map");
        return Status::Err;
    }

    system_table().con_in.reset(false);
    while let Ok(key) = system_table().con_in.read_key_blocking() {
        println!("Got key: {:?}", key);
        if key.scan_code == 23 {
            break;
        }
    }

    return Status::Success;
}

#[no_mangle]
extern "C" fn efi_main(ih: Handle, st: *mut SystemTable) -> u64 {
    efi::init(ih, st);
    return main().to_num();
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    // TODO: check if BS are available
    println!("Panic: {}!", panic);
    system_table().boot_services.exit(Status::Err);
}
