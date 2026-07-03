# Rendering

A renderer draws a frame; it never touches the core's state. The core hands back
a `PanelData` per panel (see [PANELS.md](PANELS.md)), and a front-end turns each
data-model into pixels. Rendering lives entirely **outside** `xray-core` — the
reference implementation is the web app under [`web/`](../web), but the same data
crosses every binding, so a renderer can be written in any language.

## The web renderer

`web/` is a Vue 3 + Vite app over the WASM binding. It instantiates
`bindings/wasm` client-side and drives it through the JSON command protocol —
`setSpec` / `load` / `frame` / `frameAt` — exactly like the CLI. There is **no
backend, no API key and no network feed**: it is a static, historical explorer.

```
XrayFrame (JSON)  ──►  PanelGrid.vue  ──►  one <canvas> per panel
                            │  dispatch by PanelData.kind
                            ▼
        src/render/{footprint,heatmap,liquidation_map,funding_oi_divergence}.ts
                            │  draw*(ctx, data, width, height)
                            ▼
                        pixels
```

Each renderer is a pure function `draw*(ctx, data, w, h)` that reads only its
`PanelData` and paints a `CanvasRenderingContext2D`:

- **footprint** — diverging buy/sell bars per price bin.
- **heatmap** — the dense `time × price` matrix as a dark→blue intensity ramp.
- **liquidation_map** — a marker at each `(ts, price_bin)`, radius ∝ `qty`, colour by side.
- **funding_oi_divergence** — the three aligned series as stacked, normalised lanes.

## Scrubbing

The `Scrubber` component is a range slider bound to a cursor timestamp. As the
user drags, the app calls `frameAt(ts)` and `PanelGrid` repaints every panel from
the new frame — the time-machine path described in [STREAMING.md](STREAMING.md).
Because the renderers only read `PanelData`, a repaint is just "draw the new
arrays"; there is no incremental state to keep in sync.

## Writing your own renderer

Any consumer can render: load a dataset, ask for a frame, and read the panel
arrays. The `PanelData` kind tag (`footprint`, `book_heatmap`, `liquidation_map`,
`funding_oi_divergence`) tells you which shape to expect, and the arrays are
already length-aligned and monotonic — no defensive checks needed. The web
renderers are ~40 lines each and are the clearest worked example.
