#![no_main]
//! Fuzz the order-book fold: an arbitrary sequence of book events is applied to
//! a `BookState` and queried. No sequence — however adversarial (crossed books,
//! empty sides, extreme prices) — may panic.

use libfuzzer_sys::fuzz_target;
use xray_core::{BookEvent, BookState};

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(events) = serde_json::from_str::<Vec<BookEvent>>(text) else {
        return;
    };
    if events.len() > 5000 {
        return;
    }
    let mut book = BookState::new();
    for event in &events {
        book.apply(event);
        if let Some(mid) = book.mid() {
            // Query a bin around the current mid; the bounds may be adversarial.
            let _ = book.resting_in_bin(mid - 1.0, mid + 1.0);
        }
    }
});
