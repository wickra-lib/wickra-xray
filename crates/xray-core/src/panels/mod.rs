//! Panel builders — fold a windowed [`Dataset`](crate::dataset::Dataset) into
//! the render data-models in [`frame`](crate::frame).
//!
//! Each builder is a pure function of the already-windowed dataset plus its
//! spec parameters, so it is deterministic and free of shared state. The
//! `build_panel_data` dispatcher that routes an `XrayPanel` to the right
//! builder lands once all four builders exist (§1.9).

pub mod book_heatmap;
pub mod footprint;
pub mod funding_oi_divergence;
pub mod liquidation_map;
