# Benchmarks

An X-Ray's cost is dominated by folding a dataset (trades, order-book diffs,
funding/OI) into the four microstructure panels and materialising an `XrayFrame`.
The benchmarks here measure that **core frame-build work**, so throughput scales
predictably with the dataset size and the panel configuration a spec references.

## What is measured

The `xray-bench` crate (criterion) covers frame construction across a matrix of:

- **Dataset size** — the number of events (trades + book diffs + funding/OI)
  folded before the frame is built.
- **Panels** — specs enabling one panel vs all four.
- **Access pattern** — a full fold to the last timestamp vs a `frame_at(ts)`
  seek to an interior timestamp.

## Methodology

Run against fixed, in-process synthetic datasets so the numbers are reproducible
and contain no I/O variance:

```bash
cargo bench -p xray-bench
```

## Results

Measured on one developer machine (release build, `parallel` feature), median
criterion estimates. Treat these as orders of magnitude, not guarantees — they
vary with CPU and toolchain.

`build_frame`, full fold to the last event:

| Events | 1 panel (footprint) | 4 panels (all) |
|--------|--------------------:|---------------:|
| 1k     |            ~0.19 ms |         ~15 ms |
| 10k    |             ~2.8 ms |          ~2.0 s |
| 100k   |              ~33 ms | (multi-second) |

The single-panel footprint fold is roughly linear in the event count (~0.3 µs per
trade). The four-panel figure is dominated by the `book_heatmap` panel, whose
dense `time × price` intensity matrix grows with **both** axes; on large datasets
a coarser `bucket_ms` / `price_bin` (fewer time columns and price rows) keeps it
bounded. `frame` and `frame_at(ts)` share the same fold cost for the same window.

## Caveats

These figures bound the frame-build overhead only. End-to-end time in a real run
also depends on loading the dataset from disk or a live feed, which these
in-process benchmarks do not capture.
