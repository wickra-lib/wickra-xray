# Wickra X-Ray — WASM

WASM bindings for the `wickra-xray` data-driven core, compiled to WebAssembly
with wasm-bindgen. Build an `Xray` from a spec JSON, drive it with command JSON,
read back render frames — the same protocol as every other binding, running in
the browser. This is the binding the reference `web/` front-end renders.

The core is built with `--no-default-features`, so the panels build
**sequentially** (no rayon thread pool in the browser sandbox) and byte-identical
to the native parallel build.

## Build

```bash
wasm-pack build --target web
```

This emits `pkg/` with the `.wasm` module and JS glue.

## Usage

```js
import init, { Xray, version } from "./pkg/wickra_xray_wasm.js";

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

## API

| Member | Description |
|--------|-------------|
| `new Xray(specJson)` | Build an xray from a spec JSON (throws on an invalid spec). |
| `xray.command(cmdJson)` | Apply a command JSON, return the response JSON. |
| `xray.version()` / `version()` | The library version. |

## License

`MIT OR Apache-2.0`.
