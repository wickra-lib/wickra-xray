//! WebAssembly bindings for `wickra-xray` (wasm-bindgen).
//!
//! The data-driven frame core, compiled to WebAssembly for the browser: build an
//! `Xray` from a spec JSON, drive it with a command JSON and read back the
//! response JSON. The same command protocol crosses every binding, and this is
//! the binding the reference `web/` front-end renders.
//!
//! The `parallel` feature of the core is disabled here: rayon's thread pool is
//! not available in a browser sandbox, so the panels build sequentially — which
//! is byte-identical to the parallel build, the exact cross-language golden
//! check.

use wasm_bindgen::prelude::*;

use xray_core::Xray as CoreXray;

/// An xray instance driven by JSON commands.
#[wasm_bindgen]
pub struct Xray {
    inner: CoreXray,
}

#[wasm_bindgen]
impl Xray {
    /// Build an xray from a spec JSON string.
    #[wasm_bindgen(constructor)]
    pub fn new(spec_json: &str) -> Result<Xray, JsError> {
        CoreXray::new(spec_json)
            .map(|inner| Self { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Apply a command JSON and return the resulting response JSON.
    pub fn command(&mut self, cmd_json: &str) -> Result<String, JsError> {
        self.inner
            .command_json(cmd_json)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// The library version.
    #[wasm_bindgen(js_name = version)]
    pub fn instance_version(&self) -> String {
        CoreXray::version().to_string()
    }
}

/// The library version.
#[wasm_bindgen]
pub fn version() -> String {
    CoreXray::version().to_string()
}
