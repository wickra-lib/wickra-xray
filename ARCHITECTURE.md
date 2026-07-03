# Architecture

`wickra-xray` is one data-driven core with many thin consumers. A view is a
piece of **data** — an `XrayFrame`, built from a serde `XraySpec` folded over a
recorded dataset (trades, order-book diffs, funding and open interest). Because
the frame is data, not renderer commands, the exact same view is produced
natively, across the C ABI, in WASM and in the browser, byte-for-byte identical.

## The layers

```
CONSUMERS   CLI: crates/xray-cli   ·   web/ (Vue + Canvas)   ·   any language via its binding (command JSON)
      ▲ XrayFrame JSON                                                             ▲
CORE  crates/xray-core:  XraySpec (JSON) → Dataset fold (trades / book / funding·OI)
                         → four microstructure panels → XrayFrame → frame_at(ts)
      ▼ data-driven JSON API in ten languages (like backtest run_json / terminal command_json)
BINDINGS  python · node · wasm · c (C-ABI hub) → c / c++ / c# / go / java / r
CORES  wickra-core (indicators) · wickra-data (Candle / CSV) · [feature "live"] wickra-exchange
```

Each binding ships the same surface — an `Xray` handle plus
`command(json) -> json` and `version` — with its own README, tests, a runnable
example, and a completeness guard. The `web/` front-end consumes the identical
`XrayFrame` and draws it to a canvas.

## The core is data-driven

An `XrayFrame` is a serde data-model, not a list of renderer instructions. It
carries the four microstructure panels as plain values (bins, intensities,
levels, series), so a Python, Go or browser consumer draws the same frame a Rust
consumer would. Renderer commands cannot cross the C ABI or a WASM boundary; a
serde data-model can.

## The four panels

- **Footprint** — per-price-bin buy/sell volume over a time window.
- **Order-book heatmap** — resting quantity per (time, price) cell, reconstructed
  by folding book snapshots and diffs (a diff with `qty == 0` removes a level).
- **Liquidation map** — liquidation events placed by price and size over time.
- **Funding / OI divergence** — index-aligned funding-rate and open-interest
  series and their divergence.

## The command boundary

Every consumer talks to the core through a single JSON-in / JSON-out function,
`Xray::command`. The binding does no logic of its own — it forwards the command
string and returns the core's response verbatim. That verbatim pass-through is
what makes the golden corpus a **cross-language** parity corpus: the same command
produces a byte-identical frame in every language, with no per-language JSON
reformatting.

## Time-machine: frame_at

A dataset is folded once; `frame_at(ts)` returns the `XrayFrame` as of a
timestamp, considering only events with `ts <= t`. Empty bins/buckets are emitted
as `0.0` (not omitted) so the frame shape is stable and comparable across
timestamps — the deterministic basis for the golden corpus and for the web
scrubber that seeks through history.

## Indicators come from the Wickra core

No indicator mathematics lives in this repository. Where a panel needs a derived
series, `IndicatorSet` resolves each building block from the `wickra-core`
registry by name and parameters (the same resolver the backtester uses), so
`wickra-xray` inherits all 514 indicators and any future additions for free.

## Integration with the rest of Wickra

`wickra-xray` sits beside the other Wickra consumers — the terminal, the
screener, the backtester and the exchange layer — over the same core. It depends
on `wickra-core` (indicators) and `wickra-data` (`Candle` + CSV); the optional
`live` feature pulls `wickra-exchange` to source a live dataset. It only reads and
visualises market data — it never places orders and holds no order-secret
material.
