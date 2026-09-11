<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra X-Ray — a market-microstructure explorer over 514 streaming indicators" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/ci.svg)](https://github.com/wickra-lib/wickra-xray/actions/workflows/ci.yml)
[![CodeQL](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/codeql.svg)](https://github.com/wickra-lib/wickra-xray/actions/workflows/codeql.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-xray)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/release.svg)](https://github.com/wickra-lib/wickra-xray/releases/latest)
[![crates.io](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/crates.svg)](https://crates.io/crates/wickra-xray)
[![PyPI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/pypi.svg)](https://pypi.org/project/wickra-xray/)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/npm.svg)](https://www.npmjs.com/package/wickra-xray)
[![NuGet](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/nuget.svg)](https://www.nuget.org/packages/Wickra.Xray)
[![Maven Central](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/maven.svg)](https://central.sonatype.com/artifact/org.wickra/wickra-xray)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-xray-go)
[![R-universe](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/r-universe.svg)](https://wickra-lib.r-universe.dev)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/license.svg)](#license)
[![OpenSSF Scorecard](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/scorecard.svg)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-xray)
[![OpenSSF Best Practices](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/best-practices.svg)](https://www.bestpractices.dev)
[![Build provenance](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/provenance.svg)](https://github.com/wickra-lib/wickra-xray/attestations)
[![Docs](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/docs.svg)](https://xray.wickra.org)
[![Verified across 10 languages](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/verified.svg)](golden/)
[![Live demo](https://img.shields.io/badge/live%20demo-live.wickra.org-3b82f6)](https://live.wickra.org)

---

# Wickra X-Ray

**A free explorer that shows, historically, what only Wickra computes — footprint, order-book heatmap, liquidation map and funding/OI divergence.**

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

> **Part of the [Wickra ecosystem](https://github.com/wickra-lib):** the same data-driven core and ten-language binding surface also power [wickra-exchange](https://github.com/wickra-lib/wickra-exchange), [wickra-backtest](https://github.com/wickra-lib/wickra-backtest), [wickra-terminal](https://github.com/wickra-lib/wickra-terminal) and 20 more — see [the full list](https://github.com/wickra-lib).

Wickra X-Ray is one data-driven core, [`wickra-xray-core`](crates/wickra-xray-core): a serde
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

```rust
use wickra_xray_core::{build_frame, Dataset, Xray, XraySpec};

// The full window: fold the whole dataset and build every panel the spec names.
let spec: XraySpec = XraySpec::from_json(spec_json)?;
let mut dataset = Dataset::from_json(dataset_json)?;
dataset.sort();
let cursor = dataset.bounds().map_or(0, |(_, hi, _)| hi);
let frame = build_frame(&dataset, &spec, cursor)?;

// Scrubbing: the same spec and dataset, folded to any moment. `frame_at(t)`
// returns exactly what the full window would return over a dataset ending at t.
let mut xray = Xray::new(spec_json)?;
xray.command_json(&format!(r#"{{"cmd":"load","dataset":{dataset_json}}}"#))?;
let earlier = xray.command_json(r#"{"cmd":"frame_at","ts":1700000000000}"#)?;
```

## Status

**0.1.1 — the current release.** The core, the CLI, all ten language bindings, the
web renderer, the byte-exact golden corpus, property + fuzz tests, benchmarks and
one runnable example per language are in place and green across the full CI matrix
(10 languages × 3 OS). [ROADMAP.md](ROADMAP.md) has what is done, what is open and
what is not planned.

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
crates/wickra-xray-core    the data-driven core (XraySpec, Dataset, panels, build_frame, command_json)
crates/xray-cli     the CLI (bin: wickra-xray)
crates/wickra-xray-bench   criterion benchmarks
bindings/{python,node,wasm,c,go,csharp,java,r}   the ten-language surface
web/                the Vue + Canvas browser renderer (over the WASM binding)
golden/             a deterministic dataset, specs, and byte-exact expected frames
fuzz/               cargo-fuzz targets (spec_parse, dataset_parse, build_frame, book_fold)
examples/           one runnable "build a frame" example per language
```

## Building everything from source

The Rust core, the CLI and the WASM package build from the workspace; each
binding has its own toolchain and builds on its own.

```bash
# Rust core, CLI, C ABI, WASM crate
cargo build --workspace --all-features

# The fuzz crate is a detached workspace (cargo-fuzz builds it with sanitizer
# flags on nightly), so --workspace does not reach it.
cargo check --manifest-path fuzz/Cargo.toml

# Python: an abi3 wheel via maturin
python -m venv .venv && . .venv/bin/activate
pip install maturin pytest
maturin develop --release -m bindings/python/Cargo.toml

# Node.js: a native addon via napi-rs
( cd bindings/node && npm ci && npm run build )

# WASM: a browser/bundler package via wasm-pack
wasm-pack build bindings/wasm --target bundler --out-dir pkg

# C ABI: the cdylib and staticlib every non-native binding links against
cargo build -p wickra-xray-c --release

# C / C++: the example harness, which is also the header smoke test
cmake -S examples/c -B examples/c/build && cmake --build examples/c/build

# C# / Go / Java / R link the C ABI above; each needs it built first
( cd bindings/csharp && dotnet build )
( cd bindings/go && go build ./... )
( cd bindings/java && mvn -q package )
R CMD INSTALL bindings/r

# The web front-end that renders the frames
( cd web && npm ci && npm run build )
```

The R package resolves the C ABI itself: `configure` downloads the
`wickra-xray-c-<triple>.tar.gz` asset matching its version and bundles the
shared library. Set `WKXRAY_INC` and `WKXRAY_LIB` to build against a locally
built one instead.

## Testing

```bash
# The core, in both build paths — they must agree byte for byte
cargo test --workspace --all-features
cargo test --workspace --no-default-features

cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
cargo fmt --all --check
cargo deny check

# Per binding
"$(pwd)/.venv/bin/python" -m pytest bindings/python/tests -q
( cd bindings/node && npm test )
( cd bindings/go && go test ./... )
( cd bindings/java && mvn -q test )
( cd bindings/csharp && dotnet test )
Rscript bindings/r/tests/run_tests.R
Rscript bindings/r/tests/golden.R          # golden parity: run from the repo root
ctest --test-dir examples/c/build --output-on-failure

# Fuzzing (nightly)
cargo +nightly fuzz run spec_parse -- -max_total_time=30
```

Every binding runs the same committed corpus under [`golden/`](golden/) and has
to reproduce each expected frame **byte for byte** — that is what makes the
cross-language claim checkable rather than asserted. Regenerating those files is
the bless loop in [`golden/README.md`](golden/README.md), and the diff is meant
to be read before it is committed.

## Requirements

- **Rust** ≥ 1.86 (workspace MSRV; the Node binding needs ≥ 1.88).
- Binding toolchains as needed: Node ≥ 22, Python ≥ 3.9, a C toolchain, .NET 8,
  JDK 22+, Go 1.23, R ≥ 2.10 — see each `bindings/<lang>/README.md`.

## Benchmarks

`crates/wickra-xray-bench` measures `build_frame` scaling by event count and panel
count, parallel vs sequential. See [BENCHMARKS.md](BENCHMARKS.md).

## Ecosystem

Part of the [Wickra](https://github.com/wickra-lib/wickra) family — each one a
data-driven core with a CLI and the same ten-language binding surface:

- [**wickra**](https://github.com/wickra-lib/wickra) — main library (Rust core + Python / Node.js / WASM bindings + a C ABI for C / C++ / C# / Go / Java / R)
- [**wickra-playground**](https://github.com/wickra-lib/wickra-playground) — a polyglot strategy playground: one StrategySpec live side by side in Python, Rust, JS and Go, entirely in the browser
- [**wickra-exchange**](https://github.com/wickra-lib/wickra-exchange) — unified market-data + execution across ten crypto exchanges
- [**wickra-backtest**](https://github.com/wickra-lib/wickra-backtest) — event-driven backtester over the Wickra core
- [**wickra-terminal**](https://github.com/wickra-lib/wickra-terminal) — the trading terminal: a TUI and a browser renderer over the stack
- [**wickra-screener**](https://github.com/wickra-lib/wickra-screener) — parallel multi-symbol screening over 514 streaming indicators
- [**wickra-radar**](https://github.com/wickra-lib/wickra-radar) — perp-universe alert radar: OI delta, funding flip, book imbalance, liquidation clusters, OI/price divergence
- [**wickra-copilot**](https://github.com/wickra-lib/wickra-copilot) — local market copilot grounded in real order-book, liquidation and funding microstructure
- [**wickra-shazam**](https://github.com/wickra-lib/wickra-shazam) — match an asset's current microstructure fingerprint against its entire history
- [**wickra-benchmark**](https://github.com/wickra-lib/wickra-benchmark) — reproducible, golden-verified benchmark suite — recompute any (strategy, dataset, report) in ten languages and confirm it byte-for-byte
- [**wickra-strategy-ci**](https://github.com/wickra-lib/wickra-strategy-ci) — Jest for trading strategies: golden-pin the report, catch regressions in CI, property-test against fuzzed data
- [**wickra-verify**](https://github.com/wickra-lib/wickra-verify) — confirm or refute a claimed backtest report against its strategy and data, in ten languages
- [**wickra-proof**](https://github.com/wickra-lib/wickra-proof) — Proof-of-Backtest: deterministic (spec, data) → report + blake3 hash, recomputable byte-for-byte in ten languages
- [**wickra-zk**](https://github.com/wickra-lib/wickra-zk) — prove a backtest zero-knowledge — on-chain-verifiable performance without revealing the data or the strategy
- [**wickra-impact**](https://github.com/wickra-lib/wickra-impact) — the backtester that knows you would have moved the market: agent-based fills on the real historical L2 order book
- [**wickra-darwin**](https://github.com/wickra-lib/wickra-darwin) — evolutionary strategy search at millions of backtests per second, mutating and crossing JSON specs across the 514-indicator space
- [**wickra-gym**](https://github.com/wickra-lib/wickra-gym) — a Gymnasium-compatible, microstructure-aware backtest environment with O(1) steps for deterministic RL rollouts
- [**wickra-feature-store**](https://github.com/wickra-lib/wickra-feature-store) — OHLCV and microstructure streams into ML-ready feature matrices over 514 O(1) streaming indicators
- [**wickra-genome**](https://github.com/wickra-lib/wickra-genome) — a vector database of the whole market: every asset a 514-dim live vector, for similarity search, clustering and anomaly detection
- [**wickra-timemachine**](https://github.com/wickra-lib/wickra-timemachine) — scrub the whole market like a video — every symbol, full order book, rewound to any moment via deterministic re-fold
- [**wickra-synth**](https://github.com/wickra-lib/wickra-synth) — deterministic synthetic market microstructure: OHLCV, order book, trades and funding from a single seed
- [**wickra-compile**](https://github.com/wickra-lib/wickra-compile) — compile a strategy spec into a standalone deployable: a WASM module, a self-contained binary, or a `no_std` artifact
- [**wickra-embed**](https://github.com/wickra-lib/wickra-embed) — allocation-free, `no_std` streaming indicators for bare-metal and HFT, byte-for-byte identical to the core
- [**wickra-pico**](https://github.com/wickra-lib/wickra-pico) — the O(1) indicator core running bare-metal on a $5 Raspberry Pi Pico — the LED blinks on the EMA cross

Docs at [docs.wickra.org](https://docs.wickra.org); the marketing site and
in-browser demo at [wickra.org](https://wickra.org).

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

---

<p align="center">
  <a href="https://github.com/wickra-lib/wickra-xray">
    <img alt="GitHub stars" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/stars.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-xray/network/members">
    <img alt="GitHub forks" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/forks.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-xray/issues">
    <img alt="GitHub issues" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/issues.svg">
  </a>
</p>

<p align="center">
  Built on <a href="https://github.com/wickra-lib/wickra">Wickra</a>. If it saved you time, the cheapest way to say thanks is to ⭐ the repo.
</p>

<p align="center">
  <img alt="wickra-xray star history" width="640"
       src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/star-history.svg">
</p>
