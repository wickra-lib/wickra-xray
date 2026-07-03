//! The CLI config file — an [`XraySpec`] wrapper (§1.15).
//!
//! The CLI reads a JSON or TOML config with a top-level `spec` table and hands
//! the validated spec to the core. Parsing validates the spec, so a malformed
//! config is rejected at load time rather than mid-build.

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::spec::XraySpec;

/// A parsed config file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    /// The X-Ray spec to run.
    pub spec: XraySpec,
}

impl Config {
    /// Parse and validate a config from JSON.
    pub fn from_json(s: &str) -> Result<Self> {
        let config: Config = serde_json::from_str(s)?;
        config.spec.validate()?;
        Ok(config)
    }

    /// Parse and validate a config from TOML.
    pub fn from_toml(s: &str) -> Result<Self> {
        let config: Config = toml::from_str(s)?;
        config.spec.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_json_config() {
        let config = Config::from_json(
            r#"{ "spec": {
                "dataset_ref": "mini", "symbol": "AAA",
                "panels": [ { "kind": "footprint", "price_bin": 1.0, "bucket_ms": 60000 } ]
            } }"#,
        )
        .unwrap();
        assert_eq!(config.spec.symbol, "AAA");
        assert_eq!(config.spec.panels.len(), 1);
    }

    #[test]
    fn parses_toml_config() {
        let config = Config::from_toml(
            r#"
            [spec]
            dataset_ref = "mini"
            symbol = "AAA"
            [[spec.panels]]
            kind = "footprint"
            price_bin = 1.0
            bucket_ms = 60000
            "#,
        )
        .unwrap();
        assert_eq!(config.spec.symbol, "AAA");
    }

    #[test]
    fn rejects_an_invalid_spec() {
        // Empty panels fails validation.
        let err = Config::from_json(
            r#"{ "spec": { "dataset_ref": "m", "symbol": "AAA", "panels": [] } }"#,
        );
        assert!(err.is_err());
    }
}
