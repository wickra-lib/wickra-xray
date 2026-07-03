# Panels

A spec lists one or more panels; the frame returns one `PanelData` per spec
panel, in the same order. Each panel is a pure aggregation of the dataset streams
into a **render data-model** — arrays a front-end draws directly, never a draw
command. The four panel kinds and their outputs (`crates/xray-core/src/panels/`):

## `footprint`

Traded volume per price bin, split by aggressor side.

```json
{ "kind": "footprint", "price_bin": 1.0, "bucket_ms": 60000 }
```

- **Input** — the `trades` stream. Each trade's price is snapped to a bin
  (`floor(price / price_bin) * price_bin`) and its quantity added to the buy or
  sell total by aggressor `side`.
- **Output** — `FootprintData { price_bins, buy_vol, sell_vol }`. `price_bins` is
  ascending and deduped; only bins with non-zero volume appear; the three arrays
  are length-aligned.

## `book_heatmap`

Resting liquidity over a time × price grid.

```json
{ "kind": "book_heatmap", "price_bin": 0.5, "bucket_ms": 2000, "depth_levels": 8 }
```

- **Input** — the `book` stream (a snapshot followed by diffs), folded through a
  `BookState`; sampled into `bucket_ms` time columns and `price_bin` rows around
  the mid, keeping `depth_levels` levels per side.
- **Output** — `HeatmapData { time, price, intensity }` — a dense `[T][P]` matrix
  where `intensity[t][p]` is the resting quantity in that cell (`0.0` when empty).
  `time` and `price` are both ascending; `intensity.len() == time.len()` and each
  row's length equals `price.len()`.

## `liquidation_map`

Liquidation events clustered by price bin.

```json
{ "kind": "liquidation_map", "price_bin": 1.0 }
```

- **Input** — the `liquidations` stream; each event's price is snapped to a bin.
- **Output** — `LiqMapData { events }`, where each `LiqEvent` is
  `{ ts, price_bin, qty, side }` (`side` is `long`/`short`). Events stay
  event-granular for tooltips and are sorted by `(ts, price_bin, side)`; the
  renderer clusters them by bin.

## `funding_oi_divergence`

Funding rate, open interest and price on one shared time axis.

```json
{ "kind": "funding_oi_divergence", "bucket_ms": 2000 }
```

- **Input** — the `funding`, `oi` and (from candles/trades) price streams,
  carried forward into `bucket_ms` buckets.
- **Output** — `DivergenceData { time, funding, oi, price }` — three
  index-aligned series on the `time` axis (`funding.len() == oi.len() ==
  price.len() == time.len()`). The divergence itself is derived by the consumer.

## Invariants

Every panel upholds, for any dataset: parallel arrays are length-aligned, each
axis (`price_bins`, `time`, `price`, event `ts`) is monotonic, and no value is
`NaN`/`inf`. These are pinned by `tests/proptest_invariants.rs`.
