Проект по минимальному хэлло ворлду под Linux i386. 

Работает c `rustc_codegen_gcc` [1724042e228c3](https://github.com/rust-lang/rustc_codegen_gcc/commit/1724042e228c331832d12cf9f198696c750f51ba) от Wed Sep 14 09:22:50 2022, патченному `gcc` [e7f055bf1cf7](https://github.com/antoyo/gcc/commit/e7f055bf1cf7fe0266710f029476f7955f328fa3) от Sun Jul 24 17:06:48 2022.

Патчи для поддержки `i386` в `rustc_codegen_gcc`:

1. `rustc_codegen_gcc_i386.patch`,
2. `rustc_codegen_gcc_flags.patch`.

Предполагается, что `rustc_codegen_gcc` лежит в `~/rustc_codegen_gcc`, а `gcc` установлен в `~/rustc_codegen_gcc/gcc/install`

- chapter_1 - no_std хэлло ворлд
- chapter_2 - no_core бинарь, но с libc
- chapter_3 - no_core, пустой `_start`
- chapter_4 - no_core x86_64 хэлло ворлд на сисколах
- chapter_5 - сишный i386 хэлло ворлд на сисколах
- chapter_6 - no_core i386 хэлло ворлд на сисколах
