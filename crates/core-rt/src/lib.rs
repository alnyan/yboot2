#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::wrapping_sub;

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
        let diff = wrapping_sub(*(a.offset(i)), *(b.offset(i)));
        if diff != 0u8 {
            return diff as i32;
        }
        i += 1;
    }
    return 0;
}
