---
name: Performance regression
about: Building a frame got slower, or scales worse than it should
title: "[perf] "
labels: performance
---

**What got slower**
- Path: [ ] frame build (`command` → `frame`) · [ ] dataset load · [ ] a binding boundary
- Panel: [ ] footprint · [ ] heatmap · [ ] liquidation map · [ ] funding/OI
- Build: [ ] parallel (rayon) · [ ] sequential (the WASM fallback)

**Measurements**

| | Before | After |
|---|---|---|
| Version / commit | | |
| Time per frame | | |
| Events in the dataset | | |
| Price bins × time buckets | | |
| Panels in the spec | | |

**How you measured**
The command, the harness, and whether the machine was otherwise idle. `cargo
bench` output or a CodSpeed link is ideal — note that CodSpeed reports
instruction counts under simulation, not wall-clock.

**The spec**

```json
# the XraySpec you measured
```

**Why you think it is a regression rather than the workload**
Frame-build cost should stay roughly linear in events and in grid cells. A change
in shape — the same dataset costing more per event, or the sequential build
diverging from the parallel one — is the interesting part.

**Environment**
- OS, CPU and core count:
- Rust toolchain (`rustc -V`):
- Language / binding:
