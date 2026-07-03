//! Property-based invariants: for random datasets and random (valid) specs,
//! `build_frame` never panics and every panel obeys its structural contract —
//! panel count matches the spec, each panel's parallel arrays are length-aligned,
//! every axis is monotonic, and no value is `NaN`/`inf`.
//!
//! The parallel-vs-sequential byte-equality is a compile-time property (the
//! `parallel` feature switches the fold with no runtime toggle), so it is pinned
//! by running the golden suite under both `--all-features` and
//! `--no-default-features` in CI rather than inside this single-binary test.

use proptest::prelude::*;
use xray_core::{
    build_frame, BookEvent, BookKind, Candle, Dataset, FundingEvent, LiqSide, LiquidationEvent,
    OiEvent, PanelData, Side, Trade, XrayPanel, XraySpec,
};

fn arb_side() -> impl Strategy<Value = Side> {
    prop_oneof![Just(Side::Buy), Just(Side::Sell)]
}

fn arb_liq_side() -> impl Strategy<Value = LiqSide> {
    prop_oneof![Just(LiqSide::Long), Just(LiqSide::Short)]
}

fn arb_book_kind() -> impl Strategy<Value = BookKind> {
    prop_oneof![Just(BookKind::Snapshot), Just(BookKind::Diff)]
}

fn arb_price() -> impl Strategy<Value = f64> {
    1.0f64..10_000.0
}

fn arb_qty() -> impl Strategy<Value = f64> {
    0.0f64..1_000.0
}

fn arb_ts() -> impl Strategy<Value = i64> {
    0i64..100_000
}

fn arb_candle() -> impl Strategy<Value = Candle> {
    (
        arb_ts(),
        arb_price(),
        arb_price(),
        arb_price(),
        arb_price(),
        arb_qty(),
    )
        .prop_map(|(ts, open, high, low, close, volume)| Candle {
            ts,
            open,
            high,
            low,
            close,
            volume,
        })
}

fn arb_trade() -> impl Strategy<Value = Trade> {
    (arb_ts(), arb_price(), arb_qty(), arb_side()).prop_map(|(ts, price, qty, side)| Trade {
        ts,
        price,
        qty,
        side,
    })
}

fn arb_level() -> impl Strategy<Value = [f64; 2]> {
    (arb_price(), arb_qty()).prop_map(|(price, qty)| [price, qty])
}

fn arb_book() -> impl Strategy<Value = BookEvent> {
    (
        arb_ts(),
        arb_book_kind(),
        prop::collection::vec(arb_level(), 0..4),
        prop::collection::vec(arb_level(), 0..4),
    )
        .prop_map(|(ts, kind, bids, asks)| BookEvent {
            ts,
            kind,
            bids,
            asks,
        })
}

fn arb_funding() -> impl Strategy<Value = FundingEvent> {
    (arb_ts(), -0.01f64..0.01).prop_map(|(ts, rate)| FundingEvent { ts, rate })
}

fn arb_oi() -> impl Strategy<Value = OiEvent> {
    (arb_ts(), 0.0f64..1_000_000.0).prop_map(|(ts, oi)| OiEvent { ts, oi })
}

fn arb_liquidation() -> impl Strategy<Value = LiquidationEvent> {
    (arb_ts(), arb_price(), arb_qty(), arb_liq_side()).prop_map(|(ts, price, qty, side)| {
        LiquidationEvent {
            ts,
            price,
            qty,
            side,
        }
    })
}

fn arb_dataset() -> impl Strategy<Value = Dataset> {
    (
        prop::collection::vec(arb_candle(), 0..8),
        prop::collection::vec(arb_trade(), 0..12),
        prop::collection::vec(arb_book(), 0..8),
        prop::collection::vec(arb_funding(), 0..6),
        prop::collection::vec(arb_oi(), 0..6),
        prop::collection::vec(arb_liquidation(), 0..6),
    )
        .prop_map(
            |(candles, trades, book, funding, oi, liquidations)| Dataset {
                candles,
                trades,
                book,
                funding,
                oi,
                liquidations,
            },
        )
}

fn arb_panel() -> impl Strategy<Value = XrayPanel> {
    prop_oneof![
        (0.01f64..100.0, 1i64..120_000).prop_map(|(price_bin, bucket_ms)| XrayPanel::Footprint {
            price_bin,
            bucket_ms
        }),
        (0.01f64..100.0, 1i64..120_000, 1u32..20).prop_map(
            |(price_bin, bucket_ms, depth_levels)| {
                XrayPanel::BookHeatmap {
                    price_bin,
                    bucket_ms,
                    depth_levels,
                }
            }
        ),
        (0.01f64..100.0).prop_map(|price_bin| XrayPanel::LiquidationMap { price_bin }),
        (1i64..120_000).prop_map(|bucket_ms| XrayPanel::FundingOiDivergence { bucket_ms }),
    ]
}

fn arb_spec() -> impl Strategy<Value = XraySpec> {
    (
        prop::collection::vec(arb_panel(), 1..5),
        prop::option::of((0i64..50_000, 0i64..50_000)),
    )
        .prop_map(|(panels, window)| {
            // Keep `from_ts <= to_ts` so the spec always validates and the
            // Ok-path invariants are exercised (out-of-order bounds are a
            // domain error covered by the conformance suite).
            let (from_ts, to_ts) = match window {
                Some((first, second)) => (Some(first.min(second)), Some(first.max(second))),
                None => (None, None),
            };
            XraySpec {
                dataset_ref: "ds".to_owned(),
                symbol: "SYM".to_owned(),
                from_ts,
                to_ts,
                panels,
            }
        })
}

fn is_ascending(values: &[f64]) -> bool {
    values.windows(2).all(|pair| pair[0] <= pair[1])
}

fn is_ascending_i64(values: &[i64]) -> bool {
    values.windows(2).all(|pair| pair[0] <= pair[1])
}

fn all_finite(values: &[f64]) -> bool {
    values.iter().all(|value| value.is_finite())
}

/// Assert one panel's structural contract.
fn check_panel(panel: &PanelData) -> Result<(), TestCaseError> {
    match panel {
        PanelData::Footprint(data) => {
            prop_assert_eq!(data.buy_vol.len(), data.price_bins.len());
            prop_assert_eq!(data.sell_vol.len(), data.price_bins.len());
            prop_assert!(is_ascending(&data.price_bins));
            prop_assert!(all_finite(&data.price_bins));
            prop_assert!(all_finite(&data.buy_vol));
            prop_assert!(all_finite(&data.sell_vol));
        }
        PanelData::BookHeatmap(data) => {
            prop_assert_eq!(data.intensity.len(), data.time.len());
            for row in &data.intensity {
                prop_assert_eq!(row.len(), data.price.len());
                prop_assert!(all_finite(row));
            }
            prop_assert!(is_ascending_i64(&data.time));
            prop_assert!(is_ascending(&data.price));
            prop_assert!(all_finite(&data.price));
        }
        PanelData::LiquidationMap(data) => {
            let times: Vec<i64> = data.events.iter().map(|event| event.ts).collect();
            prop_assert!(is_ascending_i64(&times));
            for event in &data.events {
                prop_assert!(event.price_bin.is_finite());
                prop_assert!(event.qty.is_finite());
            }
        }
        PanelData::FundingOiDivergence(data) => {
            prop_assert_eq!(data.funding.len(), data.time.len());
            prop_assert_eq!(data.oi.len(), data.time.len());
            prop_assert_eq!(data.price.len(), data.time.len());
            prop_assert!(is_ascending_i64(&data.time));
            prop_assert!(all_finite(&data.funding));
            prop_assert!(all_finite(&data.oi));
            prop_assert!(all_finite(&data.price));
        }
    }
    Ok(())
}

proptest! {
    #[test]
    fn build_frame_upholds_the_structural_contract(
        dataset in arb_dataset(),
        spec in arb_spec(),
        cursor in 0i64..100_000,
    ) {
        // A validated spec always builds; a panic here is a real defect.
        let frame = build_frame(&dataset, &spec, cursor).expect("a valid spec must build");
        prop_assert_eq!(frame.panels.len(), spec.panels.len());
        for panel in &frame.panels {
            check_panel(panel)?;
        }
    }
}
