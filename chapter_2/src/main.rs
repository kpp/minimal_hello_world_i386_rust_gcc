#![feature(no_core)]
#![feature(lang_items)]
#![no_core]

// Говорим компилятору влинковать libc
#[cfg(target_os = "linux")]
#[link(name = "c")]
extern "C" {}

// Функция `main` на самом деле не точка входа, а вот `start` - да.
#[lang = "start"]
fn start<T>(_main: fn() -> T, _argc: isize, _argv: *const *const u8) -> isize {
    42
}

// Втыкаем символ, чтобы не получить ошибку undefined reference to `main'
fn main() {}

// Нужно компилятору
#[lang = "sized"]
pub trait Sized {}
