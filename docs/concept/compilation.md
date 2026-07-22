# Multi-Language Engine Pipeline using LLVM IR
This project demonstrates a development pipeline for a multi-language engine leveraging **C++** for library and **Lua** for scripting with **LLVM IR** as the intermediate representation for linking and cross-platform compilation.

## Pipeline Overview

### 1. C++ Library

* **Modules**: Core, Audio, Rendering, and other low-level engine components.
* **Testing**: Unit tests are written using **GTest**.
* **Compilation**: Once tests pass, C++ code is compiled into LLVM IR using **Clang**:

  ```bash
  clang++ -O2 -emit-llvm -c audio.cpp -o audio.bc
  ```

### 2. LLVM IR Linking

* C++ LLVM bitcode linked into a single module:

  ```bash
  llvm-link audio.bc render.bc ecs.bc scheduler.bc -o engine.bc
  ```

### 3. Global Optimizations

* Apply LLVM-level optimizations across all modules:

  ```bash
  opt -O3 engine.bc -o engine_opt.bc
  ```

### 4. Binary Generation (Cross-Platform)

* Generate platform-specific assembly and binaries:

**Linux x86\_64:**

```bash
llc -march=x86-64 engine_opt.bc -o engine.s
clang engine.s -o engine_linux
```

**Windows x86\_64 (cross-compile):**

```bash
llc -march=x86-64-windows engine_opt.bc -o engine_win.s
x86_64-w64-mingw32-clang engine_win.s -o engine_win.exe
```

**MacOS x86\_64:**

```bash
llc -march=x86-64-apple-darwin engine_opt.bc -o engine_macos.s
clang engine_macos.s -o engine_macos
```

### 5. End-to-End Testing

* After building each binary, an automated **E2E testing** suite executes the binaries to verify full engine functionality across platforms.

## Benefits

* **Cross-language interoperability**: Seamless integration via LLVM IR.
* **Optimizations across modules**: Global LLVM passes allow whole-engine optimization.
* **Cross-platform portability**: Single IR can target multiple OSes and architectures.
* **Preserves language features**: Templates, inline functions guarantees remain intact in LLVM IR.

## Notes

* Debugging should primarily be done before the LLVM IR stage for clarity.
* An optional minimal C API is exposed for external bindings (Python, Lua, etc.).
* Pipeline automation can be achieved using **CMake** and custom scripts.
