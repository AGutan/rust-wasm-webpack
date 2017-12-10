#![feature(lang_items)]
#![no_std]

#[no_mangle]
pub fn add_one(number: i32) -> i32 {
    number + 1
}

// Needed for no_std!
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(
    _msg: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
    _column: u32,
) -> ! {
    loop {}
}
