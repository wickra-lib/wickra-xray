<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra X-Ray — a market-microstructure explorer over 514 streaming indicators" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-xray)
[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/ci.svg)](https://github.com/wickra-lib/wickra-xray/actions/workflows/ci.yml)
[![CodeQL](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/codeql.svg)](https://github.com/wickra-lib/wickra-xray/actions/workflows/codeql.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-xray)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/license.svg)](#license)
[![OpenSSF Scorecard](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/scorecard.svg)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-xray)
[![OpenSSF Best Practices](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/best-practices.svg)](https://www.bestpractices.dev/)
[![Build provenance](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/provenance.svg)](https://github.com/wickra-lib/wickra-xray/attestations)
[![Docs](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/docs.svg)](https://wickra.org)

---

# Wickra X-Ray

**A free explorer that shows, historically, what only Wickra computes — footprint, order-book heatmap, liquidation map and funding/OI divergence.**

Wickra X-Ray is one data-driven core, [`xray-core`](crates/xray-core): a serde
`XraySpec` is folded over a recorded dataset — trades, order-book diffs, funding
and open interest — into **render data-models** (`XrayFrame`), never renderer
commands. The frames carry the four microstructure panels; a front-end just
draws them. The parallel (rayon) and sequential (the WASM fallback) builds are
**byte-for-byte identical**.

Because the frame is **data, not code**, the exact same output crosses the C ABI
and WASM unchanged. The core is exposed as a **JSON-over-C-ABI data API**
(`Xray::command`) in **Rust, Python, Node.js, WASM, C, C++, C#, Go, Java and R**,
and a **web** front-end (Vue + Canvas) renders the frames in the browser.

- **Footprint** — traded volume per price bin, split by aggressor side.
- **Order-book heatmap** — resting liquidity over a time × price grid.
- **Liquidation map** — liquidation events clustered by price bin.
- **Funding / OI divergence** — funding, open interest and price on one time axis.

## Status

**Pre-release — functionally complete, CI-verified, not yet published.** The core,
the CLI, all ten language bindings, the web renderer, the byte-exact golden
corpus, property + fuzz tests, benchmarks and one runnable example per language
are in place and green across the full CI matrix (10 languages × 3 OS). Not yet
released to any registry — track progress in [ROADMAP.md](ROADMAP.md).

## Documentation

- [Architecture](ARCHITECTURE.md) — the core, the data-driven boundary, the binding surface.
- Panel & spec reference and per-binding quickstarts under [`docs/`](docs); one runnable example per language under [`examples/`](examples).
- [ROADMAP.md](ROADMAP.md) · [BENCHMARKS.md](BENCHMARKS.md) · [THREAT_MODEL.md](THREAT_MODEL.md) · [SECURITY.md](SECURITY.md).

## Quickstart

```bash
# Build a frame from a spec + a recorded dataset, raw XrayFrame JSON
# (the same bytes every binding returns):
cargo run -p wickra-xray -- --spec golden/specs/footprint.json --stdin --format json < golden/data.json

# Human-readable summary of the frame:
cargo run -p wickra-xray -- --spec golden/specs/multi_panel.json --stdin < golden/data.json
```

The `--spec` file is an `XraySpec`; the dataset is read from `--stdin` (a JSON
`Dataset`) or from a `--data` file.

## XraySpec / panels

A spec is a JSON (or TOML) document: a `symbol`, an optional time window
(`from_ts` / `to_ts`), and a list of `panels`. Each panel names a `kind` and its
parameters; the frame returns one `PanelData` per spec panel, in order.

```json
{
  "dataset_ref": "gold",
  "symbol": "GOLD",
  "panels": [
    { "kind": "footprint", "price_bin": 1.0, "bucket_ms": 60000 },
    { "kind": "book_heatmap", "price_bin": 0.5, "bucket_ms": 2000, "depth_levels": 8 },
    { "kind": "liquidation_map", "price_bin": 1.0 },
    { "kind": "funding_oi_divergence", "bucket_ms": 2000 }
  ]
}
```

- **Panels** (`kind`): `footprint`, `book_heatmap`, `liquidation_map`, `funding_oi_divergence`.
- **Frame** — `XrayFrame { symbol, cursor_ts, panels }`; each `PanelData` is a render data-model (price bins, intensity matrices, event lists), never a draw command.

## Scrubbing through time

The frame is addressable in time. `frame` builds the full window; `frame_at(ts)`
folds only up to a cursor — the scrubber path a front-end drives as the user
drags through history. `frame_at(to_ts)` reproduces `frame` byte-for-byte, and
every panel's arrays stay length-aligned and monotonic on their axis.

## Use in any language

The same `Xray` handle — construct from a JSON spec, drive with
`command(json) -> json`, read `version` — is reachable from every binding:

```python
from wickra_xray import Xray
x = Xray('{"dataset_ref":"m","symbol":"AAA","panels":['
         '{"kind":"footprint","price_bin":1.0,"bucket_ms":60000}]}')
x.command('{"cmd":"load","dataset":{"trades":[...]}}')
frame = x.command('{"cmd":"frame"}')  # JSON XrayFrame
```

The C ABI hub (`bindings/c`) backs C, C++, C#, Go, Java and R; Rust, Python,
Node.js and WASM are native. See each `bindings/<lang>/README.md` and the runnable
[`examples/`](examples).

## Project layout

```
crates/xray-core    the data-driven core (XraySpec, Dataset, panels, build_frame, command_json)
crates/xray-cli     the CLI (bin: wickra-xray)
crates/xray-bench   criterion benchmarks
bindings/{python,node,wasm,c,go,csharp,java,r}   the ten-language surface
web/                the Vue + Canvas browser renderer (over the WASM binding)
golden/             a deterministic dataset, specs, and byte-exact expected frames
fuzz/               cargo-fuzz targets (spec_parse, dataset_parse, build_frame, book_fold)
examples/           one runnable "build a frame" example per language
```

## Building from source

```bash
cargo build --workspace
cargo test  --workspace --all-features
cargo test  --workspace --no-default-features   # sequential build path
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo run -p wickra-xray -- --spec golden/specs/footprint.json --stdin --format json < golden/data.json
```

## Requirements

- **Rust** ≥ 1.86 (workspace MSRV; the Node binding needs ≥ 1.88).
- Binding toolchains as needed: Node ≥ 22, Python ≥ 3.9, a C toolchain, .NET 8,
  JDK 22+, Go 1.23, R — see each `bindings/<lang>/README.md`.

## Benchmarks

`crates/xray-bench` measures `build_frame` scaling by event count and panel
count, parallel vs sequential. See [BENCHMARKS.md](BENCHMARKS.md).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
Commits are signed and in English; open a PR against `main`.

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). Report
vulnerabilities privately — never in a public issue.

## License

Dual-licensed under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at
your option.

## Disclaimer

Wickra X-Ray is analysis software: it computes microstructure views over
historical and live market data. It is provided "as is", without warranty of any
kind, and is **not financial advice** — it places no orders. Trading carries risk
of loss; review the code and use at your own discretion.
