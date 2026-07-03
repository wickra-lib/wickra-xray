# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- The `xray-core` data-driven core: `XraySpec` (JSON/TOML), `Dataset` (six
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
  invariants, four cargo-fuzz targets, and the `xray-bench` criterion suite.
- One runnable "build a frame" example per language and per-language guides
  under `docs/`.
- CI/CD: a multi-OS test matrix across ten languages, CodeQL, OpenSSF Scorecard,
  zizmor, link-check, benchmark and metadata-audit workflows, plus authored
  (tag-gated) release and web-deploy workflows.
- Repository scaffolding: Cargo workspace, supply-chain configuration
  (`deny.toml`, `osv-scanner.toml`, `lychee.toml`), lint configuration
  (`clippy.toml`), `repo-metadata.toml`, and dual `MIT OR Apache-2.0` licensing.

[Unreleased]: https://github.com/wickra-lib/wickra-xray/commits/main
