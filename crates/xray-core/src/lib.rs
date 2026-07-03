//! Data-driven core of the Wickra X-Ray.
//!
//! A serde `XraySpec` is folded over a recorded dataset — trades, order-book
//! diffs, funding and open interest — into an `XrayFrame`: render data-models
//! for the four microstructure panels (footprint, order-book heatmap,
//! liquidation map, funding/OI divergence). Panels build in parallel (rayon) or
//! sequentially (the WASM fallback), producing a byte-identical `XrayFrame`.
//!
//! The public surface is assembled module by module through P-XRAY-1; the final
//! re-export block lands in `lib.rs` (P-XRAY-1.16).

mod book;
mod dataset;
mod error;
mod frame;
mod indicator_set;
pub mod panels;
mod spec;
mod types;

pub use book::BookState;
pub use dataset::{Candle, Dataset};
pub use error::{Error, Result};
pub use frame::{
    DivergenceData, FootprintData, HeatmapData, LiqEvent, LiqMapData, PanelData, XrayFrame,
};
pub use spec::{XrayPanel, XrayPanelKind, XraySpec};
pub use types::{
    bin_of, round8, BookEvent, BookKind, FundingEvent, LiqSide, LiquidationEvent, OiEvent,
    OrderedF64, Side, Trade,
};
