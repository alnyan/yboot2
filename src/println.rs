use efi::system_table;
use core::fmt;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::println::do_println(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ()              => (print!("\r\n"));
    ($($arg:tt)*)   => (print!("{}\r\n", format_args!($($arg)*)));
}

pub fn do_println(args: fmt::Arguments) {
    use core::fmt::Write;
    system_table().con_out.write_fmt(args).unwrap();
}
