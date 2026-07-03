//! `frame_at` semantics: folding to the dataset end reproduces `frame`
//! byte-for-byte, and folding to a midpoint clips the window to `ts ≤ mid`.
//!
//! The golden `multi_panel` spec leaves `to_ts` open, so `frame`'s cursor is the
//! dataset's upper bound. `frame_at(to_ts)` must therefore equal `frame`
//! exactly, and `frame_at(mid)` must report `cursor_ts == mid` while folding
//! strictly fewer trades.

use std::fs;
use std::path::{Path, PathBuf};

use xray_core::{PanelData, Xray, XrayFrame};

fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("golden")
}

/// An `Xray` with the `multi_panel` spec and the shared golden dataset loaded.
fn loaded_xray() -> Xray {
    let golden = golden_dir();
    let dataset = fs::read_to_string(golden.join("data.json")).unwrap();
    let spec = fs::read_to_string(golden.join("specs").join("multi_panel.json")).unwrap();
    let mut xray = Xray::new(&spec).unwrap();
    xray.command_json(&format!(r#"{{"cmd":"load","dataset":{dataset}}}"#))
        .unwrap();
    xray
}

/// Total footprint volume across the frame (buy + sell over every footprint
/// panel). The non-footprint arm is exercised by the three other panels in the
/// `multi_panel` spec, so there is no cold match arm.
fn footprint_total(frame: &XrayFrame) -> f64 {
    frame
        .panels
        .iter()
        .map(|panel| match panel {
            PanelData::Footprint(data) => data.buy_vol.iter().chain(&data.sell_vol).sum::<f64>(),
            _ => 0.0,
        })
        .sum()
}

fn i64_field(json: &str, field: &str) -> i64 {
    let value: serde_json::Value = serde_json::from_str(json).unwrap();
    value[field].as_i64().unwrap()
}

#[test]
fn frame_at_end_equals_full_frame() {
    let mut xray = loaded_xray();
    let to_ts = i64_field(&xray.command_json(r#"{"cmd":"bounds"}"#).unwrap(), "to_ts");

    let full = xray.command_json(r#"{"cmd":"frame"}"#).unwrap();
    let at_end = xray
        .command_json(&format!(r#"{{"cmd":"frame_at","ts":{to_ts}}}"#))
        .unwrap();

    assert_eq!(full, at_end);
}

#[test]
fn frame_at_midpoint_clips_the_window() {
    let mut xray = loaded_xray();
    let bounds = xray.command_json(r#"{"cmd":"bounds"}"#).unwrap();
    let from_ts = i64_field(&bounds, "from_ts");
    let to_ts = i64_field(&bounds, "to_ts");
    let mid = (from_ts + to_ts) / 2;

    let full: XrayFrame =
        serde_json::from_str(&xray.command_json(r#"{"cmd":"frame"}"#).unwrap()).unwrap();
    let clipped: XrayFrame = serde_json::from_str(
        &xray
            .command_json(&format!(r#"{{"cmd":"frame_at","ts":{mid}}}"#))
            .unwrap(),
    )
    .unwrap();

    assert_eq!(clipped.cursor_ts, mid);
    assert_eq!(clipped.panels.len(), full.panels.len());
    // Some trades fall before the midpoint and some after, so the clipped
    // window holds a strict, non-empty subset of the full window's volume.
    let clipped_total = footprint_total(&clipped);
    let full_total = footprint_total(&full);
    assert!(
        clipped_total > 0.0,
        "midpoint window should hold some trades"
    );
    assert!(
        clipped_total < full_total,
        "clipped {clipped_total} should be less than full {full_total}"
    );
}
