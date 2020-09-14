#![feature(asm)]
#![no_main]
#![no_std]

extern crate efi;
extern crate core_rt;

use efi::{Status, Handle, SystemTable, system_table};

#[macro_use]
mod println;

fn main() -> efi::Result<()> {
    let mut desc_array = [0u8; 4096];
    let mut mmap = efi::MemoryMap::new(&mut desc_array);
    let bs = &system_table().boot_services;
    let rs = &system_table().runtime_services;

    bs.get_memory_map(&mut mmap)?;

    Ok(())
}

#[no_mangle]
extern "C" fn efi_main(ih: Handle, st: *mut SystemTable) -> u64 {
    efi::init(ih, st);
    let ret = efi::Termination::to_efi(&main());
    println!("result -> {}", ret);
    return ret;
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    // TODO: check if BS are available
    println!("Panic: {}!", panic);
    system_table().boot_services.exit(Status::Err);
}
