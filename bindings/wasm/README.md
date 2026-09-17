<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra X-Ray — a market-microstructure explorer over 514 streaming indicators" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/ci.svg)](https://github.com/wickra-lib/wickra-xray/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-xray)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/npm.svg)](https://www.npmjs.com/package/wickra-xray-wasm)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-xray/license.svg)](https://github.com/wickra-lib/wickra-xray#license)

# Wickra X-Ray — WASM

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**A free explorer that shows, historically, what only Wickra computes — for WASM. `npm install wickra-xray-wasm` — pure WebAssembly, runs anywhere a modern JS engine does.**

WASM bindings for the `wickra-xray` data-driven core, compiled to WebAssembly
with wasm-bindgen. Build an `Xray` from a spec JSON, drive it with command JSON,
read back render frames — the same protocol as every other binding, running in
the browser. This is the binding the reference `web/` front-end renders.

The core is built with `--no-default-features`, so the panels build
**sequentially** (no rayon thread pool in the browser sandbox) and byte-identical
to the native parallel build.

## Install

```bash
npm install wickra-xray-wasm
```

### Building from this repository (contributors)

```bash
wasm-pack build --target web
```

This emits `pkg/` with the `.wasm` module and JS glue.

## Quick start

```js
import init, { Xray, version } from "wickra-xray-wasm";

await init();

const spec = JSON.stringify({
  dataset_ref: "mini", symbol: "AAA",
  panels: [{ kind: "footprint", price_bin: 1.0, bucket_ms: 60000 }],
});

const xray = new Xray(spec);
xray.command(JSON.stringify({ cmd: "load", dataset: {
  trades: [{ ts: 1000, price: 100.4, qty: 2.0, side: "buy" }],
}}));
const frame = JSON.parse(xray.command(JSON.stringify({ cmd: "frame" })));

console.log(frame.symbol, frame.cursor_ts);
console.log(version());
```

### API

| Member | Description |
|--------|-------------|
| `new Xray(specJson)` | Build an xray from a spec JSON (throws on an invalid spec). |
| `xray.command(cmdJson)` | Apply a command JSON, return the response JSON. |
| `xray.version()` / `version()` | The library version. |

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of wasm-bindgen, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-xray/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-xray>
- **Docs** (guides, spec reference, cookbook): <https://xray.wickra.org>
- **Runnable example:** [`examples/wasm/`](https://github.com/wickra-lib/wickra-xray/tree/main/examples/wasm)

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
