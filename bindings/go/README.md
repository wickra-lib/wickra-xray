<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra X-Ray — the market-microstructure explorer for Go" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/ci.svg)](https://github.com/wickra-lib/wickra-xray/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-xray)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-xray-go)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/license.svg)](https://github.com/wickra-lib/wickra-xray#license)

# Wickra X-Ray — Go

---

**The market-microstructure explorer core for Go, over the Wickra C ABI hub via cgo.**

[Wickra X-Ray](https://github.com/wickra-lib/wickra-xray) turns a dataset and spec into render frames — footprint, order-book heatmap, liquidation map and funding/OI divergence panels — as data-shaped view-models. This package is the Go binding: it consumes the C ABI hub through cgo and exposes the `Xray` handle with the same JSON protocol as every other binding.

## Install

Use the published **`wickra-xray-go`** module, which bundles the prebuilt C ABI library
for every platform, so `go get` + `go build` works with no extra steps (a C
compiler is still required, as the binding uses cgo):

```bash
go get github.com/wickra-lib/wickra-xray-go
```

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


`wickra-xray-go` is generated from this directory by the release pipeline: it mirrors the
Go sources, the vendored C ABI header (`include/wickra_xray.h`) and the prebuilt
libraries under `lib/<goos>_<goarch>/`. On Windows the DLL must be discoverable at
run time (next to the executable or on `PATH`).

## Building from this repository (contributors)

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

## License

Dual-licensed under [MIT](https://github.com/wickra-lib/wickra-xray/blob/main/LICENSE-MIT)
or [Apache-2.0](https://github.com/wickra-lib/wickra-xray/blob/main/LICENSE-APACHE), at your option.
