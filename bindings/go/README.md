<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra X-Ray — a market-microstructure explorer over 514 streaming indicators" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/ci.svg)](https://github.com/wickra-lib/wickra-xray/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-xray)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-xray-go)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/license.svg)](https://github.com/wickra-lib/wickra-xray#license)

# Wickra X-Ray — Go

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**A free explorer that shows, historically, what only Wickra computes — for Go. `go get github.com/wickra-lib/wickra-xray-go` — over the C ABI via cgo, prebuilt library bundled in the module.**

[Wickra X-Ray](https://github.com/wickra-lib/wickra-xray) turns a dataset and spec into render frames — footprint, order-book heatmap, liquidation map and funding/OI divergence panels — as data-shaped view-models. This package is the Go binding: it consumes the C ABI hub through cgo and exposes the `Xray` handle with the same JSON protocol as every other binding.

## Install

Use the published **`wickra-xray-go`** module, which bundles the prebuilt C ABI
library for every platform, so `go get` + `go build` works with no extra steps
(a C compiler is still required, as the binding uses cgo):

```bash
go get github.com/wickra-lib/wickra-xray-go
```

`wickra-xray-go` is generated from this directory by the release pipeline: it mirrors
the Go sources, the vendored C ABI header (`include/wickra_xray.h`) and the prebuilt
libraries under `lib/<goos>_<goarch>/`. On Linux/macOS the library path is baked
in via rpath; on Windows the DLL must be discoverable at run time (next to the
executable or on `PATH`).

### Building from this repository (contributors)

This `bindings/go` directory is the development source. To build it directly,
compile the C ABI hub and stage the library into the per-platform directory cgo
links against:

```bash
cargo build -p wickra-xray-c --release
mkdir -p bindings/go/lib/linux_amd64                    # match your GOOS_GOARCH
cp target/release/libwickra_xray.so    bindings/go/lib/linux_amd64/   # Linux
cp target/release/libwickra_xray.dylib bindings/go/lib/darwin_arm64/  # macOS (arm64)
cp target/release/wickra_xray.dll      bindings/go/lib/windows_amd64/ # Windows
```

Then, with the library on the loader path, run `go test ./...` from this directory.

## Quick start

```go
package main

import (
	"encoding/json"
	"fmt"

	wickra "github.com/wickra-lib/wickra-xray-go"
)

func main() {
	spec := `{"dataset_ref":"mini","symbol":"AAA","panels":[{"kind":"footprint","price_bin":1.0,"bucket_ms":60000}]}`
	x, err := wickra.New(spec)
	if err != nil {
		panic(err)
	}
	defer x.Close()

	load, _ := json.Marshal(map[string]any{"cmd": "load", "dataset": map[string]any{
		"trades": []map[string]any{{"ts": 1000, "price": 100.4, "qty": 2.0, "side": "buy"}},
	}})
	x.Command(string(load))

	frame, _ := x.Command(`{"cmd":"frame"}`)
	fmt.Println(frame)
	fmt.Println(wickra.Version())
}
```

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of cgo over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-xray/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-xray>
- **Docs** (guides, spec reference, cookbook): <https://xray.wickra.org>
- **Runnable example:** [`examples/go/`](https://github.com/wickra-lib/wickra-xray/tree/main/examples/go)

Wickra X-Ray ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-xray/blob/main/SECURITY.md>.

## Disclaimer

Wickra X-Ray is analysis software: it computes microstructure views over
historical and live market data. It is provided "as is", without warranty of any
kind, and is **not financial advice** — it places no orders. Trading carries risk
of loss; review the code and use at your own discretion.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-xray/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-xray/blob/main/LICENSE-MIT) at your option.
