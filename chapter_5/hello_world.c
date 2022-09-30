int sys_write(int fd, const void *buf, int size);
void sys_exit(int status);
static int main(int argc, char **argv);
static int syscall1(int n, int a1);
static int syscall3(int n, int a1, int a2, int a3);

static const char hello[] = "Hello, world!\n";

void _Noreturn __attribute__((naked)) _start() {
  __asm volatile (
    "_start:\n"
    "  mov ecx, [esp]\n"
    "  mov eax, esp\n"
    "  add eax, 4\n"
    "  push eax\n"
    "  push ecx\n"
    "  call _start_main\n"
  );
}

void _Noreturn _start_main(int argc, char **argv) {
  int status = main(argc, argv);
  sys_exit(status);
}

static int main(int argc, char **argv)
{
  sys_write(1, hello, sizeof(hello)-1);
  return 0;
}

void _Noreturn __attribute__ ((noinline)) sys_exit(int status) {
  syscall1(1, status);
  __builtin_unreachable();
}

int __attribute__ ((noinline)) sys_write(int fd, const void *buf, int size) {
  return syscall3(4, fd, (int) buf, size);
}

static int syscall1(int n, int a1) {
  int ret;
  __asm volatile (
    "  int 0x80"
    : "=a" (ret)
    : "0" (n), "b" (a1)
    : "memory"
  );
  return ret;
}

static int syscall3(int n, int a1, int a2, int a3) {
  int ret;
  __asm volatile (
    "  int 0x80"
    : "=a" (ret)
    : "0" (n), "b" (a1), "c" (a2), "d" (a3)
    : "memory"
  );
  return ret;
}