//! Panel builders — fold a windowed [`Dataset`](crate::dataset::Dataset) into
//! the render data-models in [`frame`](crate::frame).
//!
//! Each builder is a pure function of the already-windowed dataset plus its
//! spec parameters, so it is deterministic and free of shared state.
//! [`build_panel_data`] routes an [`XrayPanel`] to the right builder, threading
//! each panel's parameters through.

pub mod book_heatmap;
pub mod footprint;
pub mod funding_oi_divergence;
pub mod liquidation_map;

use crate::dataset::Dataset;
use crate::error::Result;
use crate::frame::PanelData;
use crate::spec::XrayPanel;

/// Build the render data-model for one panel over the already-windowed dataset.
///
/// `win` is the dataset already clipped to the frame's window (`build_frame`
/// does the windowing); `cursor_ts` is threaded for API parity and reserved for
/// panels that may need the exact frame edge — the current builders derive their
/// extent from the window itself, so it is unused here.
pub fn build_panel_data(panel: &XrayPanel, win: &Dataset, _cursor_ts: i64) -> Result<PanelData> {
    let data = match *panel {
        XrayPanel::Footprint {
            price_bin,
            bucket_ms,
        } => PanelData::Footprint(footprint::build(win, price_bin, bucket_ms)),
        XrayPanel::BookHeatmap {
            price_bin,
            bucket_ms,
            depth_levels,
        } => PanelData::BookHeatmap(book_heatmap::build(win, price_bin, bucket_ms, depth_levels)),
        XrayPanel::LiquidationMap { price_bin } => {
            PanelData::LiquidationMap(liquidation_map::build(win, price_bin))
        }
        XrayPanel::FundingOiDivergence { bucket_ms } => {
            PanelData::FundingOiDivergence(funding_oi_divergence::build(win, bucket_ms))
        }
    };
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::XrayPanelKind;
    use crate::types::{Side, Trade};

    #[test]
    fn dispatches_to_the_kind_matching_builder() {
        let win = Dataset {
            trades: vec![Trade {
                ts: 1,
                price: 100.0,
                qty: 1.0,
                side: Side::Buy,
            }],
            ..Dataset::default()
        };
        let panel = XrayPanel::Footprint {
            price_bin: 1.0,
            bucket_ms: 60_000,
        };
        let data = build_panel_data(&panel, &win, 1).unwrap();
        // Check the emitted variant and payload via serde to avoid an untaken
        // (uncovered) catch-all match arm.
        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains(r#""kind":"footprint""#));
        assert!(json.contains(r#""price_bins":[100.0]"#));
        assert_eq!(panel.kind(), XrayPanelKind::Footprint);
    }
}
