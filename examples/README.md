# Wickra X-Ray examples

A runnable "build a frame" example in every language. Each one builds an X-Ray
from the same spec (a single `footprint` panel), loads a two-trade inline dataset
(`buy 2.0 @ 100.4`, `sell 0.5 @ 101.8`) and prints the version and the frame. The
examples are self-contained: the spec and trades are inline, so there is no shared
`data/` directory to load (the golden fixtures live in [`../golden/`](../golden)).

## What every example prints

Every example prints the version and the frame, for example:

```text
wickra-xray 0.1.3
{"symbol":"AAA","cursor_ts":1400,"panels":[{"kind":"footprint","price_bins":[100.0,101.0],"buy_vol":[2.0,0.0],"sell_vol":[0.0,0.5]}]}
```

## Rust — `examples/rust/`

As the CI examples job runs it, from the repository root:

```bash
cargo run -q -p wickra-xray-example
```

| Example | What it does |
| --- | --- |
| `src/main.rs` | A runnable Rust example: build a frame with the native `build_frame` API and print it. |

## C / C++ — `examples/c/`

Build the library first (`cargo build -p wickra-xray-c --release`), then build and run
the examples via CMake, as the CI C ABI job does:

```bash
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

| Example | What it does |
| --- | --- |
| `frame.c` | A minimal C example: build a frame through the wickra-xray C ABI. |
| `frame.cpp` | A minimal C++ example: build a frame through the wickra-xray C ABI. |

## C# — `examples/csharp/`

As the CI examples job runs it, from the repository root:

```bash
dotnet run --project examples/csharp/Frame
```

| Example | What it does |
| --- | --- |
| `Frame/Program.cs` | A runnable .NET example: build a frame through the binding. |

## Go — `examples/go/`

As the CI examples job runs it, from the repository root:

```bash
cd examples/go && go run .
```

| Example | What it does |
| --- | --- |
| `frame.go` | A runnable Go example: build a frame through the binding. |

## R — `examples/r/`

As the CI examples job runs it, from the repository root:

```bash
R CMD INSTALL bindings/r
Rscript examples/r/frame.R
```

| Example | What it does |
| --- | --- |
| `frame.R` | A runnable R example: build a frame through the binding. |

## Java — `examples/java/`

As the CI examples job runs it, from the repository root:

```bash
mvn -f bindings/java/pom.xml -q package -DskipTests
javac -cp bindings/java/target/classes examples/java/Frame.java -d examples/java/out
java --enable-native-access=ALL-UNNAMED  -Dnative.lib.dir="$PWD/target/release"  -cp "bindings/java/target/classes:examples/java/out" Frame
```

| Example | What it does |
| --- | --- |
| `Frame.java` | A runnable Java example: build a frame through the binding. |

## Python — `examples/python/`

As the CI examples job runs it, from the repository root:

```bash
python -m pip install --require-hashes -r .github/requirements/ci-dev-py3.txt
( cd bindings/python && maturin build --release --out dist )
python -m pip install --no-index --find-links bindings/python/dist wickra-xray
python examples/python/frame.py
```

| Example | What it does |
| --- | --- |
| `frame.py` | A runnable Python example: build a frame through the binding. |

## Node.js — `examples/node/`

As the CI examples job runs it, from the repository root:

```bash
( cd bindings/node && npm install --no-audit --no-fund && npx napi build --platform --release )
( cd examples/node && npm install --no-audit --no-fund )
node examples/node/frame.js
```

| Example | What it does |
| --- | --- |
| `frame.js` | A runnable Node.js example: build a frame through the binding. |

## WASM — `examples/wasm/`

Build the WASM package, serve the repository root, and open the page in a browser;
the module script inside it is what runs (CI parses it with `node --check`):

```bash
wasm-pack build bindings/wasm --target web
python -m http.server 8000     # then open http://localhost:8000/examples/wasm/
```

| Example | What it does |
| --- | --- |
| `frame.html` | A runnable example against this binding. |

## Example datasets

The examples are self-contained: the spec and the input are inline, so there is
no shared `data/` directory to load. The cross-language golden fixtures, which
every binding is checked against byte for byte, live in [`../golden/`](../golden).
