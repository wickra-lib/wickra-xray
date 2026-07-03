//! A runnable Rust example: build a frame with the native `build_frame` API and
//! print it.
//!
//! ```bash
//! cargo run -p wickra-xray-example
//! ```

use xray_core::{build_frame, Dataset, XraySpec};

const SPEC: &str = r#"{
    "dataset_ref": "m",
    "symbol": "AAA",
    "panels": [{"kind": "footprint", "price_bin": 1.0, "bucket_ms": 60000}]
}"#;

const DATASET: &str = r#"{
    "trades": [
        {"ts": 1000, "price": 100.4, "qty": 2.0, "side": "buy"},
        {"ts": 1400, "price": 101.8, "qty": 0.5, "side": "sell"}
    ]
}"#;

fn main() {
    let spec: XraySpec = XraySpec::from_json(SPEC).expect("valid spec");
    let mut dataset: Dataset = Dataset::from_json(DATASET).expect("valid dataset");
    dataset.sort();
    let cursor = dataset.bounds().map_or(0, |(_, hi, _)| hi);

    let frame = build_frame(&dataset, &spec, cursor).expect("build frame");

    println!("wickra-xray {}", xray_core::version());
    println!(
        "{}",
        serde_json::to_string(&frame).expect("serialize frame")
    );
    println!("  panels: {}", frame.panels.len());
}
