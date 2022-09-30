#![no_std]
#![feature(start, lang_items)]

// Говорим компилятору влинковать libc
#[cfg(target_os = "linux")]
#[link(name = "c")]

extern "C" {
  // Объявляем внешнюю функцию из libc
  fn puts(s: *const u8) -> i32;
}

#[start] // Говорим, что выполнение надо начинать с этого символа
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  unsafe {
    // В Расте строки не нуль-терминированные
    puts("Hello, world!\0".as_ptr());
  }
  return 0;
}

#[panic_handler] // Удовлетворяем компилятор
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
  loop {}
}

#[lang = "eh_personality"] // Удовлетворяем компилятор
extern "C" fn eh_personality() {}
