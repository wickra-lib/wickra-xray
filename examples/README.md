# Examples

A runnable "build a frame" example in every language. Each one builds an X-Ray
from the same spec (a single `footprint` panel), loads a two-trade inline dataset
(`buy 2.0 @ 100.4`, `sell 0.5 @ 101.8`) and prints the version and the frame. The
examples are self-contained: the spec and trades are inline, so there is no shared
`data/` directory to load (the golden fixtures live in [`../golden/`](../golden)).

| Language | Path | Run |
|----------|------|-----|
| Rust | [`rust/`](rust/) | `cargo run -p wickra-xray-example` |
| Python | [`python/frame.py`](python/frame.py) | `pip install wickra-xray && python examples/python/frame.py` |
| Node.js | [`node/`](node/) | `cd examples/node && npm install && node frame.js` |
| C / C++ | [`c/`](c/) | see below |
| Go | [`go/`](go/) | `cd examples/go && go run .` |
| C# | [`csharp/Frame/`](csharp/Frame/) | `dotnet run --project examples/csharp/Frame` |
| Java | [`java/Frame.java`](java/Frame.java) | see the header comment |
| R | [`r/frame.R`](r/frame.R) | `Rscript examples/r/frame.R` |

The native bindings (Python, Node.js) load their own compiled library. The bindings
that go through the C ABI (Go, C#, Java, R, and the C / C++ example itself) need the
C ABI library built first:

```bash
cargo build --release -p wickra-xray-c
```

## C / C++

The C and C++ examples build with CMake and run under ctest:

```bash
cargo build --release -p wickra-xray-c
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

On Windows the build copies `wickra_xray.dll` next to each executable, since there
is no rpath.

## Expected output

Every example prints the version and the frame, for example:

```text
wickra-xray 0.1.0
{"symbol":"AAA","cursor_ts":1400,"panels":[{"kind":"footprint","price_bins":[100.0,101.0],"buy_vol":[2.0,0.0],"sell_vol":[0.0,0.5]}]}
```
