//! Node.js bindings for `wickra-xray` (napi-rs).
//!
//! Thin glue over the xray core's data-driven surface: build an `Xray` from a
//! spec JSON, drive it with a command JSON and read back the response JSON. The
//! same command protocol crosses every binding, so a Node front-end drives the
//! exact same core as the native CLI.

#![allow(missing_debug_implementations)]
// napi exposes owned `String` arguments; the bodies only need to borrow them.
#![allow(clippy::needless_pass_by_value)]

use napi::Result;
use napi_derive::napi;

use xray_core::Xray as CoreXray;

/// Build a napi error from a message.
fn err(message: impl Into<String>) -> napi::Error {
    napi::Error::from_reason(message.into())
}

/// The library version.
#[napi]
pub fn version() -> String {
    CoreXray::version().to_string()
}

/// An xray instance driven by JSON commands.
#[napi]
pub struct Xray {
    inner: CoreXray,
}

#[napi]
impl Xray {
    /// Build an xray from a spec JSON string.
    #[napi(constructor)]
    pub fn new(spec_json: String) -> Result<Self> {
        CoreXray::new(&spec_json)
            .map(|inner| Self { inner })
            .map_err(|e| err(e.to_string()))
    }

    /// Apply a command JSON and return the resulting response JSON.
    #[napi]
    pub fn command(&mut self, cmd_json: String) -> Result<String> {
        self.inner
            .command_json(&cmd_json)
            .map_err(|e| err(e.to_string()))
    }

    /// The library version.
    #[napi]
    pub fn version(&self) -> String {
        CoreXray::version().to_string()
    }
}
