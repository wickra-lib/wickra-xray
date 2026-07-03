//! Data-driven core of the Wickra X-Ray.
//!
//! A serde [`XraySpec`] is folded over a recorded [`Dataset`] — trades,
//! order-book diffs, funding and open interest — into an [`XrayFrame`]: render
//! data-models for the four microstructure panels (footprint, order-book
//! heatmap, liquidation map, funding/OI divergence). These are render
//! data-models, describing *what* to draw, not renderer commands.
//!
//! Every binding drives the core through one entry point,
//! [`Xray::command_json`], whose reply is always a JSON string; the pure
//! [`build_frame`] builder sits underneath. With the `parallel` feature (on by
//! default) the panels build concurrently with rayon; `--no-default-features`
//! builds them sequentially for the WASM target — both yield a byte-identical
//! frame.
//!
//! ```
//! use xray_core::Xray;
//! let mut xray = Xray::new("").unwrap();
//! let reply = xray.command_json(r#"{"cmd":"version"}"#).unwrap();
//! assert!(reply.contains("version"));
//! ```

mod book;
mod config;
mod dataset;
mod error;
mod frame;
mod indicator_set;
pub mod panels;
mod spec;
mod types;
mod xray;

pub use book::BookState;
pub use config::Config;
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
pub use xray::{build_frame, Bounds, Xray};

/// The crate version, e.g. `"0.1.0"`.
#[must_use]
pub fn version() -> &'static str {
    Xray::version()
}
