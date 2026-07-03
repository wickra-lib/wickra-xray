# Wickra X-Ray — Go

Go bindings for the `wickra-xray` data-driven core over its C ABI hub. Build an
`Xray` from a spec JSON, drive it with command JSON, read back render frames —
the same protocol as every other binding.

## Install

```sh
go get github.com/wickra-lib/wickra-xray-go
```

The binding is cgo over the C ABI: it needs the prebuilt native library staged
under `lib/<goos>_<goarch>/` and the header under `include/` (both shipped in the
release module).

## Usage

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

## API

| Function | Description |
|----------|-------------|
| `New(specJSON string) (*Xray, error)` | Build an xray from a spec JSON (error on an invalid spec). |
| `(*Xray).Command(cmdJSON string) (string, error)` | Apply a command JSON, return the response JSON. |
| `(*Xray).Close()` | Free the handle (a finalizer also frees it). |
| `Version() string` | The library version. |

Domain errors (a bad spec, an unknown command) come back in-band as
`{"ok":false,"error":...}` JSON, not as a Go error.

## License

`MIT OR Apache-2.0`.
