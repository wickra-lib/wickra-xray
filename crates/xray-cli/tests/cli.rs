//! End-to-end verification of the `wickra-xray` binary (§2.4 verify clause).
//!
//! Runs the built binary against a fixture dataset and checks that `--format
//! json` reproduces `xray-core`'s frame byte-for-byte and that `--at` yields an
//! earlier, clipped frame.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use xray_core::{Dataset, Xray};

const SPEC: &str = r#"{ "dataset_ref": "m", "symbol": "AAA",
    "panels": [ { "kind": "footprint", "price_bin": 1.0, "bucket_ms": 60000 } ] }"#;
const TRADES: &str = r#"[ { "ts": 1000, "price": 100.4, "qty": 2.0, "side": "buy" },
    { "ts": 1100, "price": 100.9, "qty": 1.0, "side": "sell" },
    { "ts": 1200, "price": 101.2, "qty": 3.0, "side": "buy" } ]"#;

/// Write the fixture into a fresh temp directory and return `(base, spec, data)`.
fn fixture(tag: &str) -> (PathBuf, PathBuf, PathBuf) {
    let base = std::env::temp_dir().join(format!("wickra-xray-cli-it-{tag}"));
    let data = base.join("data");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&data).unwrap();
    let spec = base.join("spec.json");
    fs::write(&spec, SPEC).unwrap();
    fs::write(data.join("trades.json"), TRADES).unwrap();
    (base, spec, data)
}

/// The library's own frame string, for comparison with the CLI output.
fn expected_frame(at: Option<i64>) -> String {
    let mut xray = Xray::new(SPEC).unwrap();
    xray.load(Dataset::from_json(&format!(r#"{{ "trades": {TRADES} }}"#)).unwrap())
        .unwrap();
    let frame = match at {
        Some(ts) => xray.frame_at(ts).unwrap(),
        None => xray.frame().unwrap(),
    };
    serde_json::to_string(&frame).unwrap()
}

fn run_json(spec: &PathBuf, data: &PathBuf, at: Option<i64>) -> String {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_wickra-xray"));
    cmd.arg("--spec").arg(spec).arg("--data").arg(data);
    if let Some(ts) = at {
        cmd.arg("--at").arg(ts.to_string());
    }
    cmd.arg("--format").arg("json");
    let out = cmd.output().unwrap();
    assert!(out.status.success(), "binary failed: {:?}", out.status);
    String::from_utf8(out.stdout).unwrap().trim().to_owned()
}

#[test]
fn json_output_matches_build_frame_byte_for_byte() {
    let (base, spec, data) = fixture("full");
    assert_eq!(run_json(&spec, &data, None), expected_frame(None));
    fs::remove_dir_all(&base).unwrap();
}

#[test]
fn at_yields_an_earlier_clipped_frame() {
    let (base, spec, data) = fixture("at");
    let clipped = run_json(&spec, &data, Some(1100));
    assert_eq!(clipped, expected_frame(Some(1100)));
    // Only the ts<=1100 trades are in range, so the cursor moved back.
    assert!(clipped.contains(r#""cursor_ts":1100"#));
    assert!(!clipped.contains(r#""cursor_ts":1200"#));
    fs::remove_dir_all(&base).unwrap();
}
