<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra X-Ray — a market-microstructure explorer over 514 streaming indicators" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-xray)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

<!-- Skeleton README (P-XRAY-0.12). The full ~20-badge block (CI, CodeQL, codecov,
     crates.io/PyPI/npm/NuGet/Maven/Go/R-universe, Scorecard, Best-Practices,
     Provenance, Docs, Verified) and the finished sections are assembled in
     P-XRAY-8.1, once the per-product badge SVGs are generated in the .github repo
     (P-XRAY-8.2). Until then this stays link-clean (no 404s on the repo page). -->

---

# Wickra X-Ray

**A free explorer that shows, historically, what only Wickra computes — footprint, order-book heatmap, liquidation map and funding/OI divergence.**

Wickra X-Ray is one data-driven core, `xray-core`: a serde `XraySpec` is folded
over a recorded dataset — trades, order-book diffs, funding and open-interest —
into **render data-models** (`XrayFrame`), never renderer commands. The frames
carry the four microstructure panels; a front-end just draws them.

Because the frame is **data, not code**, the exact same output crosses the C ABI
and WASM unchanged. The core is exposed as a **JSON-over-C-ABI data API**
(`Xray::command`) in **Rust, Python, Node.js, WASM, C, C++, C#, Go, Java and R**,
and a **web** front-end (Vue + Canvas) renders the frames in the browser.

## Status

**Pre-release — under active construction.** This repository is being built out
phase by phase (scaffold → core → CLI → eight language bindings → web renderer →
golden corpus → property/fuzz tests → CI → docs). It is not yet published to any
registry.

## Documentation

The full documentation — the `XraySpec` / panel reference, the frame data-model,
per-binding quickstarts and the web renderer — is finalized in this README and
under `docs/` during the documentation phase.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.

## Disclaimer

Wickra X-Ray is analysis software: it computes microstructure views over
historical and live market data. It does not provide financial advice and places
no orders. Trading carries risk; use at your own discretion.
