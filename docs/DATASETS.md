# Datasets

A dataset is the recorded market data a spec is folded over. It is a JSON object
with six optional streams; every field defaults to an empty list, so a spec that
only needs trades can load `{"trades":[...]}`. The core sorts each stream by
timestamp on load and never mutates it afterwards (`crates/xray-core/src/dataset.rs`).

```json
{
  "candles":      [{ "ts": 1000, "open": 100.0, "high": 100.6, "low": 99.7, "close": 100.4, "volume": 12.0 }],
  "trades":       [{ "ts": 1000, "price": 100.4, "qty": 2.0, "side": "buy" }],
  "book":         [{ "ts": 1000, "kind": "snapshot", "bids": [[100.0, 5.0]], "asks": [[101.0, 4.0]] }],
  "funding":      [{ "ts": 1000, "rate": 0.0001 }],
  "oi":           [{ "ts": 1000, "oi": 500.0 }],
  "liquidations": [{ "ts": 1000, "price": 100.4, "qty": 2.0, "side": "long" }]
}
```

## Streams

- **`candles`** — `Candle { ts, open, high, low, close, volume }`. One OHLCV bar; the price series a divergence panel carries forward.
- **`trades`** — `Trade { ts, price, qty, side }` with `side` in `buy`/`sell` (the aggressor). Drives the footprint.
- **`book`** — `BookEvent { ts, kind, bids, asks }` with `kind` in `snapshot`/`diff`; `bids`/`asks` are `[price, qty]` pairs. A snapshot replaces the book; a diff adjusts levels (`qty = 0` removes a level). Folded by `BookState` into the heatmap.
- **`funding`** — `FundingEvent { ts, rate }`. The periodic funding rate.
- **`oi`** — `OiEvent { ts, oi }`. Open interest snapshots.
- **`liquidations`** — `LiquidationEvent { ts, price, qty, side }` with `side` in `long`/`short`. Drives the liquidation map.

Every timestamp is milliseconds. `Candle::new` and the other constructors reject
non-finite values, so a loaded dataset never carries `NaN`/`inf` into a panel.

## Time window

A spec's optional `from_ts` / `to_ts` clip the dataset before folding; `frame_at`
narrows the upper bound further to the scrubber cursor. The effective window is
`[from_ts, min(cursor, to_ts)]`. See [STREAMING.md](STREAMING.md).

## Sourcing recorded data

The CLI reads a dataset from `--stdin` (a JSON `Dataset`) or a `--data` file. For
a live recording, the optional `live` feature pulls the
[`wickra-exchange`](https://github.com/wickra-lib/wickra-exchange) facade, which
records book snapshots/diffs, trades, funding, open interest and liquidations from
a venue into the same `Dataset` shape — so a replayed capture and a synthetic
fixture are interchangeable. The `golden/data.json` fixture is a deterministic
example of all six streams; see [`golden/README.md`](../golden/README.md).
