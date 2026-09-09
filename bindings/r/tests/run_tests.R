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

cat("wickra-xray R tests passed\n")
