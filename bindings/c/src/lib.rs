//! The wickra-xray C ABI — the hub every C-capable language links against.
//!
//! The surface is deliberately tiny and JSON-shaped, exactly like
//! `Xray::command_json`: construct an `Xray` from a spec JSON, drive it with
//! command JSONs and read back response JSONs, and free the handle. No xray type
//! crosses the boundary by value — the handle is opaque and the payloads are
//! always UTF-8 JSON strings.
//!
//! Responses use a caller-owned buffer with a length-out protocol (the classic
//! C two-call idiom), so the caller never has to free a callee allocation:
//!
//! 1. Call [`wickra_xray_command`] with `out = NULL`, `cap = 0` to learn the
//!    response length `len` (excluding the terminating NUL).
//! 2. Allocate `len + 1` bytes and call again; the response plus a NUL is
//!    written into `out`.
//!
//! Whenever `len < cap` the response is written immediately, so a
//! sufficiently-large buffer needs only one call. Negative returns are reserved
//! for unusable arguments ([`WICKRA_XRAY_ERR_NULL`], [`WICKRA_XRAY_ERR_UTF8`])
//! and caught panics ([`WICKRA_XRAY_ERR_PANIC`]); a non-negative return is always
//! the response length. Domain errors (a bad spec, an unknown command) are *not*
//! negative — they come back in-band as `{"ok":false,"error":...}` JSON in the
//! buffer.

use core::ffi::{c_char, CStr};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;

use xray_core::Xray;

/// A required pointer argument (`handle` or `cmd_json`) was null.
pub const WICKRA_XRAY_ERR_NULL: i32 = -1;
/// `cmd_json` was not valid UTF-8.
pub const WICKRA_XRAY_ERR_UTF8: i32 = -2;
/// A panic was caught at the FFI boundary.
pub const WICKRA_XRAY_ERR_PANIC: i32 = -3;

/// An opaque handle to an xray instance. Created by [`wickra_xray_new`] and
/// destroyed by [`wickra_xray_free`]; never dereferenced by the caller.
pub struct WickraXray(Xray);

/// Read a NUL-terminated C string as `&str`, or `None` on null / bad UTF-8.
///
/// # Safety
/// `ptr` must be null or a valid NUL-terminated C string.
unsafe fn opt_str<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    unsafe { CStr::from_ptr(ptr) }.to_str().ok()
}

/// Construct an xray from a spec JSON string (`""` or `"{}"` for an empty
/// handle whose spec is set later via a `set_spec` command).
///
/// Returns an opaque handle, or null if `spec_json` is null, not valid UTF-8, or
/// not a valid spec. Free the handle with [`wickra_xray_free`].
///
/// # Safety
/// `spec_json` must be null or a valid NUL-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn wickra_xray_new(spec_json: *const c_char) -> *mut WickraXray {
    let Some(json) = (unsafe { opt_str(spec_json) }) else {
        return ptr::null_mut();
    };
    match catch_unwind(AssertUnwindSafe(|| Xray::new(json))) {
        Ok(Ok(xray)) => Box::into_raw(Box::new(WickraXray(xray))),
        _ => ptr::null_mut(),
    }
}

/// Destroy an xray handle. Null is a no-op.
///
/// # Safety
/// `handle` must be null or a handle previously returned by [`wickra_xray_new`]
/// and not already freed.
#[no_mangle]
pub unsafe extern "C" fn wickra_xray_free(handle: *mut WickraXray) {
    if !handle.is_null() {
        drop(unsafe { Box::from_raw(handle) });
    }
}

/// Apply a command JSON and write the response JSON into the caller's buffer.
///
/// Returns the response length in bytes (excluding the terminating NUL), or a
/// negative error code. When the return value `len` satisfies `len < cap`, the
/// response and a trailing NUL have been written to `out`; otherwise `out` is
/// left untouched and the caller should re-call with a `cap` of at least
/// `len + 1`. Pass `out = NULL`, `cap = 0` to query the length without writing.
///
/// # Safety
/// `handle` must be a valid handle; `cmd_json` a valid NUL-terminated C string;
/// `out` either null or a writable buffer of at least `cap` bytes.
#[no_mangle]
pub unsafe extern "C" fn wickra_xray_command(
    handle: *mut WickraXray,
    cmd_json: *const c_char,
    out: *mut c_char,
    cap: usize,
) -> i32 {
    if handle.is_null() || cmd_json.is_null() {
        return WICKRA_XRAY_ERR_NULL;
    }
    let Some(cmd) = (unsafe { opt_str(cmd_json) }) else {
        return WICKRA_XRAY_ERR_UTF8;
    };
    let xray = unsafe { &mut (*handle).0 };
    let response = match catch_unwind(AssertUnwindSafe(|| xray.command_json(cmd))) {
        // `command_json` folds domain errors into `{"ok":false,...}` JSON, so a
        // top-level `Err` should not occur; surface it in-band all the same
        // rather than inventing a new negative code.
        Ok(result) => result.unwrap_or_else(|err| {
            format!(
                "{{\"ok\":false,\"error\":{}}}",
                json_string(&err.to_string())
            )
        }),
        Err(_) => return WICKRA_XRAY_ERR_PANIC,
    };

    let bytes = response.as_bytes();
    let len = bytes.len();
    if len < cap && !out.is_null() {
        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), out.cast::<u8>(), len);
            *out.add(len) = 0;
        }
    }
    i32::try_from(len).unwrap_or(i32::MAX)
}

/// The library version as a static NUL-terminated string (do not free).
#[no_mangle]
pub extern "C" fn wickra_xray_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0")
        .as_ptr()
        .cast::<c_char>()
}

/// Encode a string as a JSON string literal (quotes + minimal escaping).
fn json_string(s: &str) -> String {
    use std::fmt::Write as _;
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    const SPEC: &str =
        r#"{"dataset_ref":"m","symbol":"AAA","panels":[{"kind":"footprint","price_bin":1.0}]}"#;

    /// Read a NUL-terminated buffer written by the command call as a `String`.
    fn read_buf(buf: &[u8]) -> String {
        let cstr = CStr::from_bytes_until_nul(buf).unwrap();
        cstr.to_str().unwrap().to_string()
    }

    #[test]
    fn new_command_free_round_trip() {
        let spec = CString::new(SPEC).unwrap();
        let handle = unsafe { wickra_xray_new(spec.as_ptr()) };
        assert!(!handle.is_null());

        let cmd = CString::new(r#"{"cmd":"version"}"#).unwrap();
        // First call: query the length with a null buffer.
        let len = unsafe { wickra_xray_command(handle, cmd.as_ptr(), ptr::null_mut(), 0) };
        assert!(len > 0);

        // Second call: allocate len + 1 and read the response back.
        let mut buf = vec![0u8; usize::try_from(len).unwrap() + 1];
        let len2 = unsafe {
            wickra_xray_command(
                handle,
                cmd.as_ptr(),
                buf.as_mut_ptr().cast::<c_char>(),
                buf.len(),
            )
        };
        assert_eq!(len2, len);
        assert!(read_buf(&buf).contains("\"version\""));

        unsafe { wickra_xray_free(handle) };
    }

    #[test]
    fn too_small_buffer_leaves_out_untouched() {
        let spec = CString::new(SPEC).unwrap();
        let handle = unsafe { wickra_xray_new(spec.as_ptr()) };
        let cmd = CString::new(r#"{"cmd":"version"}"#).unwrap();

        let mut buf = vec![0xAAu8; 4]; // deliberately too small
        let len = unsafe {
            wickra_xray_command(
                handle,
                cmd.as_ptr(),
                buf.as_mut_ptr().cast::<c_char>(),
                buf.len(),
            )
        };
        assert!(usize::try_from(len).unwrap() >= buf.len());
        assert!(buf.iter().all(|&b| b == 0xAA)); // untouched

        unsafe { wickra_xray_free(handle) };
    }

    #[test]
    fn bad_command_reports_error_in_band() {
        let spec = CString::new(SPEC).unwrap();
        let handle = unsafe { wickra_xray_new(spec.as_ptr()) };
        let bad = CString::new(r#"{"cmd":"nope"}"#).unwrap();

        let len = unsafe { wickra_xray_command(handle, bad.as_ptr(), ptr::null_mut(), 0) };
        assert!(len > 0); // in-band error, not a negative code
        let mut buf = vec![0u8; usize::try_from(len).unwrap() + 1];
        unsafe {
            wickra_xray_command(
                handle,
                bad.as_ptr(),
                buf.as_mut_ptr().cast::<c_char>(),
                buf.len(),
            );
        }
        assert!(read_buf(&buf).contains("\"ok\":false"));

        unsafe { wickra_xray_free(handle) };
    }

    #[test]
    fn null_spec_yields_null_handle() {
        let handle = unsafe { wickra_xray_new(ptr::null()) };
        assert!(handle.is_null());
    }

    #[test]
    fn null_guards_on_command() {
        let cmd = CString::new(r#"{"cmd":"version"}"#).unwrap();
        // Null handle.
        let code =
            unsafe { wickra_xray_command(ptr::null_mut(), cmd.as_ptr(), ptr::null_mut(), 0) };
        assert_eq!(code, WICKRA_XRAY_ERR_NULL);
        // Null command with a valid handle.
        let spec = CString::new(SPEC).unwrap();
        let handle = unsafe { wickra_xray_new(spec.as_ptr()) };
        let code = unsafe { wickra_xray_command(handle, ptr::null(), ptr::null_mut(), 0) };
        assert_eq!(code, WICKRA_XRAY_ERR_NULL);
        unsafe { wickra_xray_free(handle) };
    }

    #[test]
    fn free_null_is_a_noop() {
        unsafe { wickra_xray_free(ptr::null_mut()) };
    }

    #[test]
    fn version_is_nul_terminated() {
        let p = wickra_xray_version();
        let v = unsafe { CStr::from_ptr(p) }.to_str().unwrap();
        assert_eq!(v, env!("CARGO_PKG_VERSION"));
    }
}
