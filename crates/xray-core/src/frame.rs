//! Render data-models — the frame the core hands back (§6.4).
//!
//! A [`XrayFrame`] describes *what* to draw (bins, matrices, events), never
//! *how* (colour, pixels) — that is the web renderer's job. The internal
//! `kind` tag matches [`XrayPanelKind`](crate::spec::XrayPanelKind) exactly, so
//! a spec panel maps 1:1 to a frame panel at the same index.
//!
//! The builders that fold a dataset into these models live in `panels/`
//! (§1.9–§1.13); this module is the shared vocabulary they populate.

use serde::{Deserialize, Serialize};

use crate::types::LiqSide;

/// One rendered frame: the panels for a symbol, folded up to `cursor_ts`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct XrayFrame {
    /// The symbol the dataset belongs to.
    pub symbol: String,
    /// The instant folded up to: `min(ts, to_ts)` for `frame_at`, else `to_ts`.
    pub cursor_ts: i64,
    /// Same length and order as `XraySpec.panels`.
    pub panels: Vec<PanelData>,
}

/// The rendered payload of one panel. The `kind` tag matches `XrayPanelKind`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PanelData {
    /// Traded volume per price bin, split by aggressor side.
    Footprint(FootprintData),
    /// Resting liquidity over a time x price grid.
    BookHeatmap(HeatmapData),
    /// Liquidation events clustered by price bin.
    LiquidationMap(LiqMapData),
    /// Funding vs. open interest vs. price over time.
    FundingOiDivergence(DivergenceData),
}

/// Footprint: traded volume per price bin, buy and sell kept apart (§6.4.1).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FootprintData {
    /// Ascending bin lower bounds (`floor(price/bin)*bin`), deduped and sorted;
    /// only bins with non-zero volume are present.
    pub price_bins: Vec<f64>,
    /// Per bin: total buy-side quantity. `len == price_bins.len()`.
    pub buy_vol: Vec<f64>,
    /// Per bin: total sell-side quantity. `len == price_bins.len()`.
    pub sell_vol: Vec<f64>,
}

/// Book heatmap: resting liquidity over a fixed time x price grid (§6.4.2).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HeatmapData {
    /// Column timestamps (bucket starts, ascending), `len == T`.
    pub time: Vec<i64>,
    /// Row price bins (ascending), `len == P`.
    pub price: Vec<f64>,
    /// Dense `[T][P]` matrix: resting quantity in the price bin at the time
    /// bucket, `0.0` when empty.
    pub intensity: Vec<Vec<f64>>,
}

/// A single liquidation, its price snapped to a bin (§6.4.3).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LiqEvent {
    /// The liquidation timestamp.
    pub ts: i64,
    /// `floor(price/price_bin)*price_bin`.
    pub price_bin: f64,
    /// Liquidated quantity.
    pub qty: f64,
    /// The liquidated side.
    pub side: LiqSide,
}

/// Liquidation map: the events stay event-granular for tooltips/scrubbing;
/// the renderer clusters them by bin (§6.4.3).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LiqMapData {
    /// Events sorted by `(ts, price_bin, side)`.
    pub events: Vec<LiqEvent>,
}

/// Funding / OI / price divergence: three index-aligned series on one time
/// axis; the divergence is derived by the consumer (§6.4.4).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DivergenceData {
    /// Bucket timestamps (ascending), the shared axis. `len == T`.
    pub time: Vec<i64>,
    /// Per bucket: the last funding rate (carry-forward). `len == T`.
    pub funding: Vec<f64>,
    /// Per bucket: the last open interest (carry-forward). `len == T`.
    pub oi: Vec<f64>,
    /// Per bucket: the last close/mid (carry-forward). `len == T`.
    pub price: Vec<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_data_tag_is_snake_case() {
        let panel = PanelData::FundingOiDivergence(DivergenceData {
            time: vec![0],
            funding: vec![0.0],
            oi: vec![0.0],
            price: vec![0.0],
        });
        let json = serde_json::to_string(&panel).unwrap();
        assert!(json.contains(r#""kind":"funding_oi_divergence""#));
    }

    #[test]
    fn frame_round_trips_through_json() {
        let frame = XrayFrame {
            symbol: "BTCUSDT".to_owned(),
            cursor_ts: 1_705_280_000_000,
            panels: vec![
                PanelData::Footprint(FootprintData {
                    price_bins: vec![100.0, 101.0],
                    buy_vol: vec![1.5, 0.0],
                    sell_vol: vec![0.0, 2.0],
                }),
                PanelData::LiquidationMap(LiqMapData {
                    events: vec![LiqEvent {
                        ts: 5,
                        price_bin: 100.0,
                        qty: 3.0,
                        side: LiqSide::Long,
                    }],
                }),
            ],
        };
        let json = serde_json::to_string(&frame).unwrap();
        let back: XrayFrame = serde_json::from_str(&json).unwrap();
        assert_eq!(frame, back);
    }
}
