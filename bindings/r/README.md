<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra X-Ray — a market-microstructure explorer over 514 streaming indicators" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/ci.svg)](https://github.com/wickra-lib/wickra-xray/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-xray)
[![r-universe](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/r-universe.svg)](https://wickra-lib.r-universe.dev)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/license.svg)](https://github.com/wickra-lib/wickra-xray#license)

# Wickra X-Ray — R

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**A free explorer that shows, historically, what only Wickra computes — for R. `install.packages("wickraxray", repos = "https://wickra-lib.r-universe.dev")` — over the C ABI via `.Call`, prebuilt library fetched on install.**

R bindings for the `wickra-xray` data-driven core, over its C ABI hub (`.Call`).
Build an xray from a spec JSON, drive it with command JSON, read back render
frames — the same protocol as the CLI and every other binding.

## Install

From r-universe:

```r
install.packages("wickraxray", repos = "https://wickra-lib.r-universe.dev")
```

The package's `configure` downloads the prebuilt C ABI library for this exact
version from the GitHub release and bundles it, so an ordinary install needs
nothing but a C toolchain (Rtools on Windows) for the thin `.Call` glue layer. To
build against a local checkout instead, point it at the header and library with
the environment variables below.

### Building from this repository (contributors)

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

## Quick start

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

### API

| Function | Description |
|----------|-------------|
| `wkxray_new(spec_json)` | Build an xray from a spec JSON (errors on an invalid spec). |
| `wkxray_command(xray, cmd_json)` | Apply a command JSON, return the response JSON. |
| `wkxray_version()` | The library version. |

Domain errors (a bad spec, an unknown command) come back in-band as
`{"ok":false,"error":...}` JSON.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of R's native `.Call` interface over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-xray/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-xray>
- **Docs** (guides, spec reference, cookbook): <https://xray.wickra.org>
- **Runnable example:** [`examples/r/`](https://github.com/wickra-lib/wickra-xray/tree/main/examples/r)

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
