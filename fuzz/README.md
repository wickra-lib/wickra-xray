# Fuzzing Wickra X-Ray

[`cargo-fuzz`](https://rust-fuzz.github.io/book/cargo-fuzz.html) harnesses for the parsing and stateful entry points of Wickra X-Ray. Fuzzing requires a nightly Rust toolchain; CI runs every target for 30 seconds on the family's pinned `nightly-2026-07-01`.

## Setup

```bash
cargo install cargo-fuzz
rustup toolchain install nightly-2026-07-01
```

The date is the family's fuzz nightly, pinned in `ci.yml`: a rolling `nightly`
regressed with a codegen ICE unrelated to this code, so every repository moves
the date together, on purpose.

## Targets

| Target | What it exercises |
| --- | --- |
| `spec_parse` | The spec-parsing path: arbitrary bytes are parsed as an X-Ray spec (JSON and TOML) and as a config. |
| `dataset_parse` | The dataset-parsing path: arbitrary bytes are parsed as a recorded dataset. |
| `build_frame` | The full frame build: a `{spec, dataset, cursor_ts}` object is parsed and folded into a frame. |
| `book_fold` | The order-book fold: an arbitrary sequence of book events is applied to a `BookState` and queried. |

## Run

```bash
# From the repository root:
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu spec_parse
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu dataset_parse
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu build_frame
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu book_fold
```

Each run continues until a crash is found or it is interrupted. A short
time-boxed smoke run is what CI does:

```bash
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu spec_parse -- -max_total_time=30
```

The expectation for every target is that it never panics: malformed or
adversarial input must surface as an `Err` or an in-band error, never a crash.
