//! The `XraySpec` — the data-driven input describing a symbol, a time window and
//! the panels to build (§6.2 / §6.3).

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Which microstructure panel a spec entry selects. The discriminant matches the
/// `PanelData` kind tag one-to-one.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum XrayPanelKind {
    /// Per-price-bin buy/sell volume.
    Footprint,
    /// Resting book quantity per (time, price) cell.
    BookHeatmap,
    /// Liquidation events by price and size.
    LiquidationMap,
    /// Funding-rate vs open-interest divergence.
    FundingOiDivergence,
}

fn def_bin() -> f64 {
    1.0
}
fn def_bucket() -> i64 {
    60_000
}
fn def_depth() -> u32 {
    40
}

/// A panel entry with its per-kind parameters. Internally tagged by `kind`, so
/// each panel carries exactly the parameters it needs.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum XrayPanel {
    /// Per-price-bin buy/sell volume.
    Footprint {
        /// Price bin width (`> 0`).
        #[serde(default = "def_bin")]
        price_bin: f64,
        /// Time bucket width in ms (`> 0`).
        #[serde(default = "def_bucket")]
        bucket_ms: i64,
    },
    /// Resting book quantity per (time, price) cell.
    BookHeatmap {
        /// Price bin width (`> 0`).
        #[serde(default = "def_bin")]
        price_bin: f64,
        /// Time bucket width in ms (`> 0`).
        #[serde(default = "def_bucket")]
        bucket_ms: i64,
        /// Number of bins around the mid per column (`> 0`).
        #[serde(default = "def_depth")]
        depth_levels: u32,
    },
    /// Liquidation events by price and size.
    LiquidationMap {
        /// Price bin width (`> 0`).
        #[serde(default = "def_bin")]
        price_bin: f64,
    },
    /// Funding-rate vs open-interest divergence.
    FundingOiDivergence {
        /// Time bucket width in ms (`> 0`).
        #[serde(default = "def_bucket")]
        bucket_ms: i64,
    },
}

/// Reject a parameter that is not a finite positive number.
fn require_positive(value: f64, what: &str) -> Result<()> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(Error::BadSpec(format!("{what} must be a positive number")))
    }
}

impl XrayPanel {
    /// The kind discriminant of this panel.
    #[must_use]
    pub fn kind(&self) -> XrayPanelKind {
        match self {
            XrayPanel::Footprint { .. } => XrayPanelKind::Footprint,
            XrayPanel::BookHeatmap { .. } => XrayPanelKind::BookHeatmap,
            XrayPanel::LiquidationMap { .. } => XrayPanelKind::LiquidationMap,
            XrayPanel::FundingOiDivergence { .. } => XrayPanelKind::FundingOiDivergence,
        }
    }

    fn validate(&self) -> Result<()> {
        match *self {
            XrayPanel::Footprint {
                price_bin,
                bucket_ms,
            } => {
                require_positive(price_bin, "footprint price_bin")?;
                if bucket_ms <= 0 {
                    return Err(Error::BadSpec("footprint bucket_ms must be > 0".into()));
                }
            }
            XrayPanel::BookHeatmap {
                price_bin,
                bucket_ms,
                depth_levels,
            } => {
                require_positive(price_bin, "book_heatmap price_bin")?;
                if bucket_ms <= 0 {
                    return Err(Error::BadSpec("book_heatmap bucket_ms must be > 0".into()));
                }
                if depth_levels == 0 {
                    return Err(Error::BadSpec(
                        "book_heatmap depth_levels must be > 0".into(),
                    ));
                }
            }
            XrayPanel::LiquidationMap { price_bin } => {
                require_positive(price_bin, "liquidation_map price_bin")?;
            }
            XrayPanel::FundingOiDivergence { bucket_ms } => {
                if bucket_ms <= 0 {
                    return Err(Error::BadSpec(
                        "funding_oi_divergence bucket_ms must be > 0".into(),
                    ));
                }
            }
        }
        Ok(())
    }
}

/// The data-driven input to a build.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct XraySpec {
    /// Logical dataset name; the CLI / web resolves it to files.
    pub dataset_ref: String,
    /// The symbol, e.g. `"BTCUSDT"`.
    pub symbol: String,
    /// Inclusive window start in ms; `None` = dataset start.
    #[serde(default)]
    pub from_ts: Option<i64>,
    /// Inclusive window end in ms; `None` = dataset end.
    #[serde(default)]
    pub to_ts: Option<i64>,
    /// The panels to build (`>= 1`); the frame keeps this order.
    pub panels: Vec<XrayPanel>,
}

impl XraySpec {
    /// Parse and validate a spec from JSON.
    pub fn from_json(s: &str) -> Result<Self> {
        let spec: XraySpec = serde_json::from_str(s)?;
        spec.validate()?;
        Ok(spec)
    }

    /// Parse and validate a spec from TOML.
    pub fn from_toml(s: &str) -> Result<Self> {
        let spec: XraySpec = toml::from_str(s)?;
        spec.validate()?;
        Ok(spec)
    }

    /// Structural validation: non-empty symbol and panels, positive panel
    /// parameters, and a well-ordered window.
    pub(crate) fn validate(&self) -> Result<()> {
        if self.symbol.is_empty() {
            return Err(Error::BadSpec("symbol must not be empty".into()));
        }
        if self.panels.is_empty() {
            return Err(Error::BadSpec("panels must not be empty".into()));
        }
        if let (Some(from), Some(to)) = (self.from_ts, self.to_ts) {
            if from > to {
                return Err(Error::BadSpec("from_ts must be <= to_ts".into()));
            }
        }
        for panel in &self.panels {
            panel.validate()?;
        }
        Ok(())
    }
}
