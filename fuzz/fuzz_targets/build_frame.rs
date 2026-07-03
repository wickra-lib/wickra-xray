#![no_main]
//! Fuzz the full frame build: a `{spec, dataset, cursor_ts}` object is parsed
//! and folded into a frame. The spec, the dataset and the cursor are all
//! attacker-controlled; the build must never panic (a bad spec is a clean
//! `Err`, not a crash).

use libfuzzer_sys::fuzz_target;
use serde::Deserialize;
use xray_core::{build_frame, Dataset, XraySpec};

#[derive(Deserialize)]
struct Input {
    spec: XraySpec,
    dataset: Dataset,
    cursor_ts: i64,
}

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(input) = serde_json::from_str::<Input>(text) else {
        return;
    };
    // Bound the total work so the fuzzer cannot request an unbounded fold.
    if input.dataset.n_events() > 5000 {
        return;
    }
    let _ = build_frame(&input.dataset, &input.spec, input.cursor_ts);
});
