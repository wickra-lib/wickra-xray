## Golden-corpus cases for the wickra-xray R binding.
##
## Lives outside run_tests.R and outside the built tarball (see .Rbuildignore).
## The corpus is at the repository root, above this package, so these cases only
## resolve when run from there -- which CI does explicitly. `R CMD check` runs
## everything under tests/ from the built tarball, where the corpus does not
## exist, and that is what r-universe runs on every platform: a shipped test that
## reasons about the repository above it passes for the wrong reason at best and
## fails every platform binary at worst.
##
## Run from the repository root:
##
##     Rscript bindings/r/tests/golden.R

library(wickraxray)

## cross-language golden parity: build the xray from each committed
## golden/specs/*.json, load the shared golden/data.json and read back the frame,
## and assert the response equals golden/expected/<spec>.json byte-for-byte. The
## binding returns the core's compact command output verbatim, so byte equality
## is the exact cross-language parity check.
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
if (is.null(g)) {
  stop("golden fixtures not found; run this from the repository root")
}

dataset <- trimws(paste(
  readLines(file.path(g, "data.json"), warn = FALSE), collapse = "\n"
))
load_all <- paste0('{"cmd":"load","dataset":', dataset, '}')
specs <- list.files(file.path(g, "specs"), pattern = "\\.json$", full.names = TRUE)
if (length(specs) == 0) {
  stop("golden/specs holds no spec; the parity check would assert nothing")
}
for (spec_path in specs) {
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

cat("wickra-xray R golden parity passed (", length(specs), " specs )\n", sep = "")
