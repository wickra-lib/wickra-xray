# Wickra X-Ray — C / C++ examples

The Wickra X-Ray C ABI is a single shared/static library plus a generated header
([`bindings/c/include/wickra_xray.h`](../../bindings/c/include/wickra_xray.h)). Any C-capable
language links against the same artifact; these examples show the plain-C path
and, through [`wickra_xray.hpp`](../../bindings/c/include/wickra_xray.hpp), the C++ one.

## Build the library

From the workspace root:

```sh
cargo build -p wickra-xray-c --release
```

This produces, in `target/release/`:

| Platform | Shared library | Link target |
|----------|----------------|-------------|
| Linux    | `libwickra_xray.so`     | `-lwickra_xray` |
| macOS    | `libwickra_xray.dylib`  | `-lwickra_xray` |
| Windows (MSVC) | `wickra_xray.dll` | `wickra_xray.dll.lib` (import lib) |

A static library (`libwickra_xray.a` / `wickra_xray.lib`) is emitted alongside.

## Build and run the examples

### With CMake (portable, used by CI)

```sh
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

### Directly with a compiler

```sh
# Linux / macOS
cc examples/c/frame.c -I bindings/c/include -L target/release -lwickra_xray -lm -o frame
LD_LIBRARY_PATH=target/release ./frame        # macOS: DYLD_LIBRARY_PATH

# Windows (MinGW gcc, linking the DLL directly)
gcc examples/c/frame.c -I bindings/c/include target/release/wickra_xray.dll -lm -o frame.exe
```

## The examples

| Example | What it does |
|---------|--------------|
| `frame.c` | A minimal C example: build a frame through the wickra-xray C ABI. |
| `frame.cpp` | A minimal C++ example: build a frame through the wickra-xray C ABI. |

## Usage shape

Every call follows the same handle discipline: construct from a spec JSON, drive
with command JSON, read the response, free the handle exactly once. `wickra_xray.h` is
the whole contract; the C++ header, where one ships, wraps the handle in a
move-only RAII type. See [`bindings/c/README.md`](../../bindings/c/README.md).
