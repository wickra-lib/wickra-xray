# wickra-xray WASM examples

Browser demos for the `wickra-xray-wasm` binding.

The WASM build carries the whole frame core with `--no-default-features`: no
rayon, so the fold is sequential rather than parallel — and byte-for-byte
identical to the parallel one, which is what the golden fixtures pin down. A
frame is data rather than draw calls, so the spec bytes on this page are the same
ones `examples/node/frame.js` sends, and the page is free to render them however
it likes.

## Build

The module ships as a `wasm-pack` `--target web` bundle. Build it once from the
repository root:

```bash
wasm-pack build bindings/wasm --target web --release
```

That writes `bindings/wasm/pkg/` with the `.wasm` binary, the JS loader and the
TypeScript types. The demo imports the loader via
`../../bindings/wasm/pkg/wickra_xray_wasm.js`.

## Serve

ES-module imports need a real HTTP origin, not `file://`. Any static server from
the repository root works:

```bash
python -m http.server 8000
```

Then open `http://localhost:8000/examples/wasm/frame.html`.

## Demos

| File | What it does |
| --- | --- |
| `frame.html` | Builds an X-Ray from the shared spec (one `footprint` panel), loads the two inline trades, reads back the frame and renders the footprint bins as a table, plus the raw `XrayFrame` JSON. The page counterpart of `examples/node/frame.js`. |

The full renderer — all four panels on canvas, over real market data — is the web
front-end under [`web/`](../../web), live at [live.wickra.org](https://live.wickra.org).

## See also

- [examples/README.md](../README.md) — the same frame in every other language.
- [bindings/wasm/README.md](../../bindings/wasm/README.md) — the module's API and
  what the sequential build does and does not carry.
