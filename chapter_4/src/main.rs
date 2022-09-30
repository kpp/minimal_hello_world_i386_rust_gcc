#![feature(no_core)]
#![feature(lang_items)]
#![no_core]
#![no_main]
#![feature(naked_functions)]
#![feature(decl_macro)]
#![feature(rustc_attrs)]
#![feature(intrinsics)]

// Нужно компилятору
#[lang = "sized"]
pub trait Sized {}

#[lang = "copy"]
pub trait Copy {}

impl Copy for i64 {} // Говорим компилятору, что объект этого типа можно копировать байт за байтом
impl Copy for usize {}

#[rustc_builtin_macro]
pub macro asm("assembly template", $(operands,)* $(options($(option),*))?) {
  /* compiler built-in */
}

extern "rust-intrinsic" {
  // Чтобы компилятор знал, что есть некоторый код, которого не достичь.
  // Например, весь код после exit()
  pub fn unreachable() -> !;
}

#[no_mangle]
#[naked]
unsafe extern "C" fn _start() {
  // Стырено из книги А.В. Столярова.
  // А, простите, там код под 32 бита, в книге 2021 года.
  // Значит, не стырено.
  asm!(
    "mov rdi, [rsp]", // argc
    "mov rax, rsp",
    "add rax, 8",
    "mov rsi, rax", // argv
    "call _start_main",
    options(noreturn),
  )
}

#[no_mangle]
extern "C" fn _start_main(argc: usize, argv: *const *const u8) -> ! {
  let status = main(argc, argv);
  exit(status);
}

#[no_mangle]
fn main(_argc: usize, _argv: *const *const u8) -> i64 {
  let string = b"Hello, world!\n" as *const _ as *const u8;
  write(1, string, 14);
  return 0;
}

#[inline(never)]
#[no_mangle]
// ! - это never type, компилятор понимает, что функция никогда не возвращается
fn exit(status: i64) -> ! {
  unsafe {
    syscall1(60, status);
    unreachable()
  }
}

#[inline(never)]
#[no_mangle]
fn write(fd: i64, data: *const u8, len: i64) -> i64 {
  unsafe { syscall3(1, fd, data as i64, len) }
}

#[inline(always)]
unsafe fn syscall1(n: i64, a1: i64) -> i64 {
  let ret: i64;
  asm!(
    "syscall",
    in("rax") n,
    in("rdi") a1,
    lateout("rax") ret,
  );
  ret
}

#[inline(always)]
unsafe fn syscall3(n: i64, a1: i64, a2: i64, a3: i64) -> i64 {
  let ret: i64;
  asm!(
    "syscall",
    in("rax") n,
    in("rdi") a1,
    in("rsi") a2,
    in("rdx") a3,
    lateout("rax") ret,
  );
  ret
}
