// A runnable Go example: build a frame through the binding.
//
//	cargo build --release -p wickra-xray-c
//	# stage the library under bindings/go/lib/<goos>_<goarch>/ (CI does this)
//	cd examples/go && go run .
package main

import (
	"fmt"

	wickra "github.com/wickra-lib/wickra-xray-go"
)

const spec = `{"dataset_ref":"m","symbol":"AAA","panels":[` +
	`{"kind":"footprint","price_bin":1.0,"bucket_ms":60000}]}`

const load = `{"cmd":"load","dataset":{"trades":[` +
	`{"ts":1000,"price":100.4,"qty":2.0,"side":"buy"},` +
	`{"ts":1400,"price":101.8,"qty":0.5,"side":"sell"}]}}`

func main() {
	xray, err := wickra.New(spec)
	if err != nil {
		panic(err)
	}
	defer xray.Close()

	if _, err := xray.Command(load); err != nil {
		panic(err)
	}
	frame, err := xray.Command(`{"cmd":"frame"}`)
	if err != nil {
		panic(err)
	}

	fmt.Println("wickra-xray", wickra.Version())
	fmt.Println(frame)
}
