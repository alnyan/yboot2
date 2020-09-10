#![no_main]
#![no_std]

extern crate efi;
extern crate core_rt;

use efi::{Status, Handle, SystemTable, system_table};

#[macro_use]
mod println;

fn main() -> Status {
    let con_out = system_table().con_out();

    print!("Firmware vendor: ");
    con_out.output_char16_string(system_table().firmware_vendor);
    print!(", revision: {}.{}",
        system_table().firmware_revision >> 16,
        system_table().firmware_revision & 0xFFFF);
    println!();

    return Status::Success;
}

#[no_mangle]
extern "C" fn efi_main(_ih: Handle, st: *mut SystemTable) -> isize {
    efi::init_tables(st);
    return main().to_isize();
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
