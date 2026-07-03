# A runnable R example: build a frame through the binding.
#
#   cargo build -p wickra-xray-c --release
#   export WKXRAY_LIB="$PWD/target/release"
#   export LD_LIBRARY_PATH="$WKXRAY_LIB:$LD_LIBRARY_PATH"   # PATH on Windows
#   R CMD INSTALL bindings/r
#   Rscript examples/r/frame.R

library(wickraxray)

spec <- paste0(
  '{"dataset_ref":"m","symbol":"AAA","panels":[',
  '{"kind":"footprint","price_bin":1.0,"bucket_ms":60000}]}'
)

load_cmd <- paste0(
  '{"cmd":"load","dataset":{"trades":[',
  '{"ts":1000,"price":100.4,"qty":2.0,"side":"buy"},',
  '{"ts":1400,"price":101.8,"qty":0.5,"side":"sell"}]}}'
)

xray <- wkxray_new(spec)
invisible(wkxray_command(xray, load_cmd))
response <- wkxray_command(xray, '{"cmd":"frame"}')

cat("wickra-xray", wkxray_version(), "\n")
cat(response, "\n")
