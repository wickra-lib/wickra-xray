//! Conformance tests: the JSON contract (§6) is stable and self-consistent —
//! every enum representation round-trips, domain errors surface as in-band JSON,
//! and the frame's panels align 1:1 with the spec's panels.

use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;

use xray_core::{
    BookKind, DivergenceData, FootprintData, HeatmapData, LiqEvent, LiqMapData, LiqSide, PanelData,
    Side, Xray, XrayFrame, XrayPanel, XrayPanelKind,
};

const SPEC: &str = r#"{ "dataset_ref": "m", "symbol": "AAA",
    "panels": [ { "kind": "footprint", "price_bin": 1.0, "bucket_ms": 60000 } ] }"#;

/// Serialize, deserialize, and assert the value survives unchanged.
fn round_trip<T: Serialize + DeserializeOwned + PartialEq + Debug>(value: &T) {
    let json = serde_json::to_string(value).unwrap();
    let back: T = serde_json::from_str(&json).unwrap();
    assert_eq!(value, &back);
}

#[test]
fn enum_tags_are_snake_case() {
    assert_eq!(serde_json::to_string(&Side::Buy).unwrap(), r#""buy""#);
    assert_eq!(serde_json::to_string(&Side::Sell).unwrap(), r#""sell""#);
    assert_eq!(serde_json::to_string(&LiqSide::Long).unwrap(), r#""long""#);
    assert_eq!(
        serde_json::to_string(&LiqSide::Short).unwrap(),
        r#""short""#
    );
    assert_eq!(
        serde_json::to_string(&BookKind::Snapshot).unwrap(),
        r#""snapshot""#
    );
    assert_eq!(serde_json::to_string(&BookKind::Diff).unwrap(), r#""diff""#);
    assert_eq!(
        serde_json::to_string(&XrayPanelKind::FundingOiDivergence).unwrap(),
        r#""funding_oi_divergence""#
    );
}

#[test]
fn scalar_enums_round_trip() {
    for side in [Side::Buy, Side::Sell] {
        round_trip(&side);
    }
    for side in [LiqSide::Long, LiqSide::Short] {
        round_trip(&side);
    }
    for kind in [BookKind::Snapshot, BookKind::Diff] {
        round_trip(&kind);
    }
    for kind in [
        XrayPanelKind::Footprint,
        XrayPanelKind::BookHeatmap,
        XrayPanelKind::LiquidationMap,
        XrayPanelKind::FundingOiDivergence,
    ] {
        round_trip(&kind);
    }
}

#[test]
fn spec_panels_round_trip() {
    round_trip(&XrayPanel::Footprint {
        price_bin: 1.0,
        bucket_ms: 60_000,
    });
    round_trip(&XrayPanel::BookHeatmap {
        price_bin: 0.5,
        bucket_ms: 2_000,
        depth_levels: 8,
    });
    round_trip(&XrayPanel::LiquidationMap { price_bin: 1.0 });
    round_trip(&XrayPanel::FundingOiDivergence { bucket_ms: 2_000 });
}

#[test]
fn panel_data_variants_round_trip() {
    round_trip(&PanelData::Footprint(FootprintData {
        price_bins: vec![100.0, 101.0],
        buy_vol: vec![2.0, 3.0],
        sell_vol: vec![1.0, 0.0],
    }));
    round_trip(&PanelData::BookHeatmap(HeatmapData {
        time: vec![1000, 2000],
        price: vec![99.5, 100.0],
        intensity: vec![vec![1.0, 2.0], vec![3.0, 0.0]],
    }));
    round_trip(&PanelData::LiquidationMap(LiqMapData {
        events: vec![LiqEvent {
            ts: 5,
            price_bin: 100.0,
            qty: 3.0,
            side: LiqSide::Long,
        }],
    }));
    round_trip(&PanelData::FundingOiDivergence(DivergenceData {
        time: vec![1000, 2000],
        funding: vec![0.0001, 0.0002],
        oi: vec![500.0, 600.0],
        price: vec![100.0, 101.0],
    }));
}

#[test]
fn invalid_spec_on_construction_errors() {
    // Empty symbol and empty panels each fail validation.
    assert!(Xray::new(r#"{"dataset_ref":"m","symbol":"","panels":[]}"#).is_err());
    assert!(Xray::new(r#"{"dataset_ref":"m","symbol":"AAA","panels":[]}"#).is_err());
}

#[test]
fn bad_spec_command_yields_error_json() {
    let mut xray = Xray::new("").unwrap();
    let reply = xray
        .command_json(r#"{"cmd":"set_spec","spec":{"dataset_ref":"m","symbol":"AAA","panels":[]}}"#)
        .unwrap();
    assert!(reply.contains(r#""ok":false"#), "{reply}");
}

#[test]
fn frame_without_dataset_yields_error_json() {
    let mut xray = Xray::new(SPEC).unwrap();
    let reply = xray.command_json(r#"{"cmd":"frame"}"#).unwrap();
    assert!(reply.contains(r#""ok":false"#), "{reply}");
    assert!(reply.contains("no dataset"), "{reply}");
}

#[test]
fn frame_panels_align_with_spec_panels() {
    // A deliberately non-alphabetical two-panel spec.
    let spec = r#"{ "dataset_ref": "m", "symbol": "AAA", "panels": [
        { "kind": "liquidation_map", "price_bin": 1.0 },
        { "kind": "footprint", "price_bin": 1.0, "bucket_ms": 60000 }
    ] }"#;
    let mut xray = Xray::new(spec).unwrap();
    xray.command_json(
        r#"{"cmd":"load","dataset":{"trades":[{"ts":1,"price":100.0,"qty":1.0,"side":"buy"}]}}"#,
    )
    .unwrap();
    let raw = xray.command_json(r#"{"cmd":"frame"}"#).unwrap();

    // The kind tags appear in the spec's order (checked via the string to avoid
    // an uncovered catch-all match arm).
    let liq = raw.find("liquidation_map").unwrap();
    let footprint = raw.find("footprint").unwrap();
    assert!(liq < footprint, "panels must keep spec order: {raw}");

    // And the frame deserializes back to two panels.
    let frame: XrayFrame = serde_json::from_str(&raw).unwrap();
    assert_eq!(frame.panels.len(), 2);
}
