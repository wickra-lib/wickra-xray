# Wickra X-Ray — Node.js

Node.js bindings for
[`xray-core`](https://github.com/wickra-lib/wickra-xray), built with
[napi-rs]. The surface mirrors every other Wickra binding: build an `Xray` from
a spec JSON, drive it with command JSONs, and read back render frames.

## Install

```sh
npm install wickra-xray
```

The right prebuilt native binary is pulled in automatically as an optional
dependency for your platform.

## Usage

```js
const { Xray } = require("wickra-xray");

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
```

## Surface

- **`new Xray(spec_json)`** builds an xray from a spec JSON (`""` or `"{}"` for
  an empty handle whose spec is set later). Throws on a malformed spec.
- **`xray.command(cmd_json)`** applies a command JSON (`set_spec`, `load`,
  `frame`, `frame_at`, `bounds`, `reset`, `version`) and returns the response
  JSON. Domain errors come back in-band as `{"ok":false,"error":...}`.
- **`xray.version()` / `version()`** return the library version.

## Build from source

```sh
npm install
npm run build
npm test
```

[napi-rs]: https://napi.rs
