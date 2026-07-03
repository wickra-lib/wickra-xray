#![no_main]
//! Fuzz the dataset-parsing path: arbitrary bytes are parsed as a recorded
//! dataset. Parsing must never panic; malformed input must surface as a clean
//! `Err`. When it does parse, sorting and bounds are exercised too.

use libfuzzer_sys::fuzz_target;
use xray_core::Dataset;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(mut dataset) = Dataset::from_json(text) else {
        return;
    };
    dataset.sort();
    let _ = dataset.bounds();
    let _ = dataset.n_events();
});
