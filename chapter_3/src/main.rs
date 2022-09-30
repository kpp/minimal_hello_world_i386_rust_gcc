#![feature(no_core)]
#![feature(lang_items)]
#![no_core]
#![no_main]

#[no_mangle]
extern "C" fn _start() {}

// Нужно компилятору
#[lang = "sized"]
pub trait Sized {}
