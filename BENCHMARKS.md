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

_To be filled in from the criterion run in the test-rigor / docs phase._ Figures
will be the median estimate on a single machine; treat them as orders of
magnitude, not guarantees — they vary with CPU and toolchain.

## Caveats

These figures bound the frame-build overhead only. End-to-end time in a real run
also depends on loading the dataset from disk or a live feed, which these
in-process benchmarks do not capture.
