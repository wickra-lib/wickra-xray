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

mod error;

pub use error::{Error, Result};
