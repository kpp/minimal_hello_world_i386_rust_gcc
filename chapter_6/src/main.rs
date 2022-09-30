#![feature(no_core)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(decl_macro)]
#![feature(rustc_attrs)]
#![feature(intrinsics)]
#![no_core]
#![no_main]

#[lang = "sized"]
pub trait Sized {}

#[lang = "copy"]
pub trait Copy {}

impl Copy for i32 {}
impl Copy for usize {}

#[rustc_builtin_macro]
pub macro asm("assembly template", $(operands,)* $(options($(option),*))?) {
  /* compiler built-in */
}

extern "rust-intrinsic" {
  pub fn unreachable() -> !;
}

#[no_mangle]
#[naked]
unsafe extern "C" fn _start() {
  asm!(
    "mov ecx, [esp]",
    "mov eax, esp",
    "add eax, 4",
    "push eax",
    "push ecx",
    "call _start_main",
    options(noreturn),
  )
}

#[no_mangle]
extern "C" fn _start_main(argc: usize, argv: *const *const u8) -> ! {
  let status = main(argc, argv);
  sys_exit(status);
}

#[no_mangle]
fn main(_argc: usize, _argv: *const *const u8) -> i32 {
  let string = b"Hello, world!\n" as *const _ as *const u8;
  sys_write(1, string, 14);
  return 0;
}

#[inline(never)]
#[no_mangle]
fn sys_write(fd: i32, data: *const u8, len: i32) -> i32 {
  unsafe { syscall3(4, fd, data as _, len) }
}

#[inline(never)]
#[no_mangle]
fn sys_exit(status: i32) -> ! {
  unsafe {
    syscall1(1, status);
    unreachable()
  }
}

#[inline(always)]
unsafe extern "C" fn syscall1(n: i32, a1: i32) -> i32 {
  let ret: i32;
  asm!(
    "int 0x80",
    in("eax") n,
    in("ebx") a1,
    lateout("eax") ret,
  );
  ret
}

#[inline(always)]
unsafe fn syscall3(n: i32, a1: i32, a2: i32, a3: i32) -> i32 {
  let ret: i32;
  asm!(
    "int 0x80",
    in("eax") n,
    in("ebx") a1,
    in("ecx") a2,
    in("edx") a3,
    lateout("eax") ret,
  );
  ret
}
