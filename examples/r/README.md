# Wickra X-Ray examples — R

Runnable R examples for the [Wickra X-Ray R binding](../../bindings/r). The package compiles a thin
`.Call` glue layer against the C ABI library, so build the library and install
the package first (the CI examples job does exactly this):

```bash
cargo build -p wickra-xray-c --release
R CMD INSTALL bindings/r
```

## Run

```bash
Rscript examples/r/frame.R
```

## The examples

| Example | What it does |
|---------|--------------|
| `frame.R` | A runnable R example: build a frame through the binding. |
