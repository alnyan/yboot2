#![no_main]
#![no_std]

mod efi;

use core::ffi::c_void;
use efi::{Status, SystemTable};

#[no_mangle]
unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, cnt: usize) -> *mut u8 {
    let mut i = 0isize;
    while i < cnt as isize {
        *(dst.offset(i)) = *(src.offset(i));
        i += 1;
    }
    return dst;
}
#[no_mangle]
unsafe extern "C" fn memset(dst: *mut u8, val: i32, cnt: usize) -> *mut u8 {
    let mut i = 0isize;
    while i < cnt as isize {
        *(dst.offset(i)) = val as u8;
        i += 1;
    }
    return dst;
}
#[no_mangle]
unsafe extern "C" fn memcmp(a: *const u8, b: *const u8, cnt: usize) -> i32 {
    let mut i = 0isize;
    while i < cnt as isize {
        let diff = *(a.offset(i)) - *(b.offset(i));
        if diff != 0u8 {
            return diff as i32;
        }
        i += 1;
    }
    return 0;
}

fn main() -> Status {
    efi::con_output_string("Test\n");

    return Status::Success;
}

#[no_mangle]
extern "C" fn efi_main(_ih: *mut c_void, st: *mut SystemTable) -> isize {
    efi::init_tables(st);

    return main().to_isize();
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
