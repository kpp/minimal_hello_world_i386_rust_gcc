~/rustc_codegen_gcc/gcc/install/bin/gcc -Os -masm=intel -m32 -fno-pic -fno-asynchronous-unwind-tables -Wall -Wno-main -c hello_world.c
ld.lld-14 --no-pie --no-dynamic-linker hello_world.o -o hello_world
strip hello_world
objcopy -j.text -j.rodata hello_world
