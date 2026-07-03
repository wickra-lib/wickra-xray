# Wickra X-Ray — Web

The reference web renderer for the Wickra X-Ray: a Vue 3 + Vite front-end over
the WASM binding. It's a **static, historical explorer** — **no backend, no API
key, no network feed**. Load a recorded dataset, scrub through time, and watch
the four microstructure panels repaint from the core's `XrayFrame`.

The browser runs the **exact same core** as the CLI and every other binding: the
WASM module (`bindings/wasm`) is instantiated client-side, and the app drives it
through the single JSON command protocol.

## Panels

Each panel is a canvas renderer that draws exactly one `PanelData` render
data-model (never the core state):

| Panel | Draws |
|-------|-------|
| **Footprint** | diverging buy / sell volume per price bin |
| **Book heatmap** | the dense time × price resting-liquidity matrix |
| **Liquidation map** | markers at (time, price bin), sized by quantity, coloured by side |
| **Funding / OI divergence** | price, funding and open interest as aligned line lanes |

## Quickstart

```bash
# Build the WASM core the app depends on.
wasm-pack build --target web        # run in ../bindings/wasm

cd web
npm install
npm run dev                         # http://localhost:5173
```

The app loads a bundled **demo dataset** on start. Use the file picker to load
your own dataset — a JSON object of the recorded event streams
(`candles`, `trades`, `book`, `funding`, `oi`, `liquidations`) — and drag the
scrubber to fold the frame up to any timestamp.

## Build

```bash
npm run build      # vue-tsc typecheck + vite build -> dist/
npm run lint       # vue-tsc --noEmit
npm run preview    # serve the production build locally
```

`dist/` is a fully static bundle (the `.wasm` module is inlined as an asset), so
it can be served from any static host or CDN — no server, no secrets.

## License

`MIT OR Apache-2.0`.
