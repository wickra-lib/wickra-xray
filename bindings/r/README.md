# Wickra X-Ray — R

R bindings for the `wickra-xray` data-driven core, over its C ABI hub (`.Call`).
Build an xray from a spec JSON, drive it with command JSON, read back render
frames — the same protocol as the CLI and every other binding.

## Usage

```r
library(wickraxray)

spec <- paste0(
  '{"dataset_ref":"mini","symbol":"AAA","panels":[',
  '{"kind":"footprint","price_bin":1.0,"bucket_ms":60000}]}'
)

xray <- wkxray_new(spec)
wkxray_command(xray, paste0(
  '{"cmd":"load","dataset":{"trades":[',
  '{"ts":1000,"price":100.4,"qty":2.0,"side":"buy"}]}}'
))
cat(wkxray_command(xray, '{"cmd":"frame"}'), "\n")
cat(wkxray_version(), "\n")
```

## Build and test from source

The package links the `wickra_xray` C ABI, located out-of-tree via two
environment variables:

```bash
# Build the C ABI shared library first.
cargo build -p wickra-xray-c --release

export WKXRAY_INC="$PWD/bindings/c/include"
export WKXRAY_LIB="$PWD/target/release"
# The loader must also find the shared library at run time:
export LD_LIBRARY_PATH="$WKXRAY_LIB:$LD_LIBRARY_PATH"   # PATH on Windows

R CMD INSTALL bindings/r
Rscript bindings/r/tests/run_tests.R
```

## API

| Function | Description |
|----------|-------------|
| `wkxray_new(spec_json)` | Build an xray from a spec JSON (errors on an invalid spec). |
| `wkxray_command(xray, cmd_json)` | Apply a command JSON, return the response JSON. |
| `wkxray_version()` | The library version. |

Domain errors (a bad spec, an unknown command) come back in-band as
`{"ok":false,"error":...}` JSON.

## License

`MIT OR Apache-2.0`.
