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

    let status = system_table().boot_services.get_memory_map(&mut mmap);
    if status != Status::Success {
        println!("Failed to obtain EFI memory map: status={}", status.to_str());
        return Status::Err;
    }

    for item in mmap.iter().unwrap() {
        println!("Item: 0x{:016x} .. 0x{:016x}", item.begin(), item.end());
    }

    return Status::Success;
}

#[no_mangle]
extern "C" fn efi_main(_ih: Handle, st: *mut SystemTable) -> u64 {
    efi::init_tables(st);
    return main().to_num();
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
