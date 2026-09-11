# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-09-11

### Changed

- The Wickra core the optional `live` feature resolves through moved from
  `0.9.9` to `1.0.4`. It arrives transitively: `wickra-exchange` is consumed
  from git, and the rev this repository pinned required the 0.9 line. Moving
  the rev forward moved the core with it, so a graph that reached for both no
  longer risks two incompatible majors.
- The Node binding's generated loader (`index.js`) was regenerated for
  `@napi-rs/cli` 3.9.0. A failed load now reports the whole chain of attempts
  through `error.cause` rather than only the last one, and the WASI environment
  variables changed meaning: `NAPI_RS_FORCE_WASI=true` keeps native as a lazy
  fallback instead of discarding it, and `NAPI_RS_WASI_FLAVOR` selects one
  generated flavour without crossing into another. The public surface
  (`index.d.ts`) is unchanged.
- Dependency updates: serde 1.0.229, thiserror 2.0.20, clap 4.6.6, pyo3 0.29.2,
  rust_decimal 1.43.0; JUnit 6.1.3 with the Maven compiler and surefire plugins;
  the .NET test SDKs; and nine pinned GitHub Actions, of which `setup-java`
  crossed a major to v6.

### Note on 0.1.0

`0.1.0` is published on crates.io, PyPI, npm, NuGet, Maven Central and the Go
module mirror, but it has no GitHub Release page. Its release run needed two
attempts -- NuGet had no trusted-publishing policy on the first -- and by the
second attempt Maven Central and the Go mirror both refused to repeat work they
had already done, so the job that attaches the release assets never ran. Nothing
about the published 0.1.0 artefacts is wrong; the release page is simply absent,
and 0.1.1 is the first version to carry one.


## [0.1.0] - 2026-09-10

### Added

- The `wickra-xray-core` data-driven core: `XraySpec` (JSON/TOML), `Dataset` (six
  streams — candles, trades, order-book snapshots/diffs, funding, open interest,
  liquidations), the four panel builders (footprint, order-book heatmap,
  liquidation map, funding/OI divergence), `build_frame` / `frame_at`, and the
  `Xray::command_json` JSON-over-C-ABI protocol. The parallel (rayon) and
  sequential builds are byte-for-byte identical.
- `wickra-xray` CLI: build a frame from a spec + dataset (`--stdin` / `--data`,
  `--format json` or a human-readable summary).
- Ten-language surface: native Rust, Python (PyO3), Node.js (napi) and WASM
  (wasm-bindgen), plus a C ABI hub (cbindgen) backing C, C++, C#, Go, Java and R.
- A Vue + Canvas web renderer over the WASM binding — a static, historical
  explorer that scrubs through time.
- A deterministic golden corpus (dataset, specs, byte-exact expected frames) and
  cross-language byte-equality tests across every binding.
- Test rigor: conformance, golden, `frame_at == frame`, property-based
  invariants, four cargo-fuzz targets, and the `wickra-xray-bench` criterion suite.
- Golden parity and the scrubber equality are checked from every binding,
  including the C ABI and WASM: `frame_at(t)` must equal the frame over a
  dataset that ends at `t`, which is the property the scrubbing claim rests on
  and which no full-window golden fixture can see.
- One runnable "build a frame" example per language -- including a browser
  demo for the WASM binding -- and per-language guides under `docs/`.
- A header-only C++ wrapper (`wickra_xray.hpp`) over the C ABI: the handle is
  owned, the two-call length protocol is handled, and a failed call raises.
- The R package resolves the C ABI itself: `configure` / `configure.win`
  download the matching release asset and bundle the shared library, so
  `install.packages()` works without `WKXRAY_INC` / `WKXRAY_LIB` preset.
- A release gate: every publish job waits on `gate`, which refuses to run
  unless the tagged commit's CI is green, and on `guard`, which refuses to
  publish from anything but a `v*` tag. Build provenance covers the NuGet
  package, the jar and the C ABI archives alongside the crates and wheels.
- Five static checks under `scripts/` that CI runs on every push: every binding's
  surface against the C ABI header, every version declaration against every
  other, absolute links in the binding READMEs, committed licence texts per
  published package, and the R glue against the header it compiles with.
- CI/CD: a multi-OS test matrix across ten languages, CodeQL, OpenSSF Scorecard,
  zizmor, link-check, benchmark and metadata-audit workflows, plus authored
  (tag-gated) release and web-deploy workflows.
- Repository scaffolding: Cargo workspace, supply-chain configuration
  (`deny.toml`, `osv-scanner.toml`, `lychee.toml`), lint configuration
  (`clippy.toml`), `repo-metadata.toml`, and dual `MIT OR Apache-2.0` licensing.

[Unreleased]: https://github.com/wickra-lib/wickra-xray/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/wickra-lib/wickra-xray/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/wickra-lib/wickra-xray/releases/tag/v0.1.0
