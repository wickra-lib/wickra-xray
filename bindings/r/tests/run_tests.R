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

## cross-language golden parity: build the xray from each committed
## golden/specs/*.json, load the shared golden/data.json and read back the frame,
## and assert the response equals golden/expected/<spec>.json byte-for-byte. The
## binding returns the core's compact command output verbatim, so byte equality
## is the exact cross-language parity check. The fixtures arrive in a later
## phase; until then the golden section is skipped.
golden_dir <- function() {
  d <- normalizePath(getwd(), mustWork = FALSE)
  for (i in seq_len(8)) {
    g <- file.path(d, "golden")
    if (dir.exists(file.path(g, "specs"))) {
      return(g)
    }
    d <- dirname(d)
  }
  NULL
}

g <- golden_dir()
if (!is.null(g)) {
  dataset <- trimws(paste(
    readLines(file.path(g, "data.json"), warn = FALSE), collapse = "\n"
  ))
  load_all <- paste0('{"cmd":"load","dataset":', dataset, '}')
  for (spec_path in list.files(file.path(g, "specs"), pattern = "\\.json$", full.names = TRUE)) {
    name <- basename(spec_path)
    spec_json <- paste(readLines(spec_path, warn = FALSE), collapse = "\n")
    expected <- trimws(paste(
      readLines(file.path(g, "expected", name), warn = FALSE), collapse = "\n"
    ))
    gxray <- wkxray_new(spec_json)
    invisible(wkxray_command(gxray, load_all))
    got <- wkxray_command(gxray, '{"cmd":"frame"}')
    stopifnot(identical(trimws(got), expected))
  }
}

cat("wickra-xray R tests passed\n")
