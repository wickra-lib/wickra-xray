# Architecture (internals)

The top-level [ARCHITECTURE.md](../ARCHITECTURE.md) gives the high-level shape;
this page covers how the core actually turns a spec + a dataset into a frame. The
whole product is **one data-driven core** (`xray-core`) and N thin consumers — the
CLI, the web renderer and the ten language bindings — each of which only ships a
spec, loads a dataset and reads back a frame.

## The pipeline

```
XraySpec (JSON/TOML)              Dataset (JSON)
   │  parse + validate               │  parse + sort every stream by ts
   │  (non-empty symbol & panels,     ▼
   │   positive price_bin/bucket_ms) window(from_ts, min(cursor, to_ts))
   ▼                                  │
build_frame(dataset, spec, cursor) ◄──┘
   │  for each XrayPanel in spec order → build_panel_data(panel, window)
   ▼
XrayFrame { symbol, cursor_ts, panels: Vec<PanelData> }
   │  serde_json::to_string  (compact, deterministic float formatting)
   ▼
the exact bytes every binding returns from a `frame` command
```

- **`XraySpec`** (`crates/xray-core/src/spec.rs`) — `symbol`, optional `from_ts`/`to_ts`, and an ordered list of `XrayPanel`. `validate()` rejects an empty symbol, empty panels, non-positive `price_bin` and non-positive `bucket_ms`.
- **`Dataset`** (`src/dataset.rs`) — six streams (`candles`, `trades`, `book`, `funding`, `oi`, `liquidations`); `sort()` orders each by timestamp, `window(from, to)` slices inclusively, `bounds()` returns `(from_ts, to_ts, count)`.
- **`build_frame`** (`src/xray.rs`) — windows the dataset to `min(cursor_ts, to_ts)`, then folds each panel independently. The panels are order-preserving, so `frame.panels[i]` always matches `spec.panels[i]`.
- **Panels** (`src/panels/`) — one builder per kind; each returns a `PanelData` render data-model, never a draw command. See [PANELS.md](PANELS.md).

## Parallel vs sequential

With the default `parallel` feature the panels are built with rayon's indexed
`par_iter().collect()`, which preserves spec order; without it (the WASM build,
`--no-default-features`) they are built sequentially. The two are
**byte-for-byte identical** — the golden suite runs under both feature sets in CI
to prove it.

## The command protocol

Every binding drives the core through one entry point, [`Xray::command_json`],
whose envelope is `{"cmd": "..."}` and whose reply is **always a JSON string**. A
domain error is returned in-band as `{"ok":false,"error":"..."}` — never a panic
or a thrown exception. The commands are `set_spec`, `load`, `frame`, `frame_at`,
`bounds`, `reset` and `version`. Because the reply is the core's compact JSON
verbatim, the frame is byte-identical across every language.

## Data-driven boundary

The frame is **data, not code**: `PanelData` carries price bins, intensity
matrices and event lists, and a front-end draws them. That is why the same output
crosses the C ABI and WASM unchanged, and why a renderer can be written in any
language without linking the core's internals. See [RENDERING.md](RENDERING.md).
