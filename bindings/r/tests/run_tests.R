## Plain-R tests for the wickra-xray R binding (no testthat dependency).
## Mirrors the Rust/Python/Node/Go/C#/Java tests and doubles as the completeness
## guard: it exercises the full public surface (version + new + command).

library(wickraxray)

spec <- paste0(
  '{"dataset_ref":"m","symbol":"AAA","panels":[{"kind":"footprint",',
  '"price_bin":1.0,"bucket_ms":60000}]}'
)

trade <- function(ts, price, qty) {
  paste0(
    '{"ts":', ts, ',"price":', price, ',"qty":', qty, ',"side":"buy"}'
  )
}

## version
stopifnot(nzchar(wkxray_version()))

## load -> frame roundtrip
xray <- wkxray_new(spec)
load_cmd <- paste0(
  '{"cmd":"load","dataset":{"trades":[',
  trade(1000, "100.4", "2.0"), ',',
  trade(1400, "101.8", "0.5"), ']}}'
)
invisible(wkxray_command(xray, load_cmd))
raw <- wkxray_command(xray, '{"cmd":"frame"}')
stopifnot(grepl('"symbol":"AAA"', raw, fixed = TRUE))
stopifnot(grepl('"cursor_ts":1400', raw, fixed = TRUE))
stopifnot(grepl('"kind":"footprint"', raw, fixed = TRUE))

## invalid spec raises
stopifnot(inherits(try(wkxray_new("not json"), silent = TRUE), "try-error"))

## an unknown command is an in-band error, not a hard error
inband <- wkxray_command(xray, '{"cmd":"nope"}')
stopifnot(grepl('"ok":false', inband, fixed = TRUE))

## The cross-language golden parity cases live in golden.R, which is kept out
## of the built tarball: the corpus sits at the repository root, above this
## package, and a shipped test must not reason about what lies above it.

## The scrubber path. "Scrub the market like a video" is the claim the frame API
## is built around, and the golden corpus cannot check it: every blessed frame is
## a full-window frame, so the cursor never sits anywhere but the end. These
## three properties pin it down instead, each failing on a different mistake --
## an off-by-one at the upper bound, a window that rounds to a bucket boundary
## rather than to the requested timestamp, and a panel that carries state past
## the cursor and would show the future. All three read the frame as text: this
## package has no JSON dependency, and the byte-exact comparison against a
## truncated dataset is made from Python, Node and WASM where parsing is native.
scrub_spec <- paste0(
  '{"dataset_ref":"m","symbol":"AAA","panels":[{"kind":"footprint",',
  '"price_bin":1.0,"bucket_ms":60000}]}'
)
scrub <- wkxray_new(scrub_spec)
invisible(wkxray_command(scrub, paste0(
  '{"cmd":"load","dataset":{"trades":[',
  trade(1000, "100.4", "2.0"), ',',
  trade(1400, "101.8", "0.5"), ',',
  trade(2000, "102.2", "1.5"), ']}}'
)))

## The spec leaves to_ts open, so frame's cursor is the dataset's own end and
## frame_at asked for that timestamp must not produce a different frame.
stopifnot(identical(
  wkxray_command(scrub, '{"cmd":"frame_at","ts":2000}'),
  wkxray_command(scrub, '{"cmd":"frame"}')
))

## A midpoint frame reports the cursor it was asked for.
mid <- wkxray_command(scrub, '{"cmd":"frame_at","ts":1400}')
stopifnot(grepl('"cursor_ts":1400', mid, fixed = TRUE))

## ...and it carries strictly less traded volume than the full one, which is
## what proves the cursor clips the window rather than merely labelling it.
volume_total <- function(frame) {
  arrays <- regmatches(
    frame, gregexpr('"(buy_vol|sell_vol)":\\[[^]]*\\]', frame)
  )[[1]]
  numbers <- unlist(lapply(arrays, function(a) {
    body <- sub('^"[a-z_]*":\\[', "", sub("\\]$", "", a))
    if (!nzchar(body)) return(numeric(0))
    as.numeric(strsplit(body, ",", fixed = TRUE)[[1]])
  }))
  sum(numbers)
}
stopifnot(volume_total(mid) < volume_total(wkxray_command(scrub, '{"cmd":"frame"}')))

cat("wickra-xray R tests passed\n")
