# Streaming through time

The frame is addressable in time. A dataset is loaded once; then any number of
frames can be built at different cursors without reloading — the time-machine a
scrubber drives as the user drags through history.

## `frame` vs `frame_at`

- **`frame`** builds the full window. Its cursor is the spec's `to_ts`, or the
  dataset's upper bound when `to_ts` is left open.
- **`frame_at(ts)`** builds only up to `ts`. The window's effective end is
  `min(ts, to_ts)`, so a cursor past `to_ts` is clamped to the spec bound.

Both call the same `build_frame`, so they share every invariant. Two properties
are pinned by `tests/frame_at_eq_frame.rs`:

1. **`frame_at(to_ts) == frame`** — folding to the dataset end reproduces the full
   frame byte-for-byte (when the spec leaves `to_ts` open).
2. **`frame_at(mid)` clips correctly** — the returned `cursor_ts` is `mid`, and the
   fold sees only events with `ts ≤ mid` (strictly fewer trades than the full
   window, when events exist on both sides).

## Driving the scrubber

```json
{"cmd":"load","dataset": { … } }        // once
{"cmd":"bounds"}                         // → {"from_ts":…, "to_ts":…, "count":…}
{"cmd":"frame_at","ts":  12000 }         // → XrayFrame folded to ts=12000
{"cmd":"frame_at","ts":  18000 }         // drag forward — re-fold, no reload
{"cmd":"frame"}                          // the full window
```

`bounds` gives the scrubber its range (`from_ts`/`to_ts`) and total event count.
Each `frame_at` re-folds from the loaded dataset — there is no incremental state,
so seeking backwards is exactly as cheap as seeking forwards, and the frame at a
given cursor is always the same bytes regardless of the path taken to reach it.

## Reset and re-load

`reset` clears the dataset but keeps the spec; `load` replaces the dataset and
returns the new event count. A front-end that lets the user open a different
capture just calls `load` again — the spec and every panel definition stay put.

## Determinism

Because a frame is a pure fold over sorted, immutable data, the same `(spec,
dataset, cursor)` always yields the same bytes — across languages (every binding
returns the core's JSON verbatim) and across the parallel and sequential builds.
That determinism is what lets the golden corpus pin the output byte-for-byte.
