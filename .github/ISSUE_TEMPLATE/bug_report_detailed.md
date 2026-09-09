---
name: Bug report (detailed)
about: A bug that needs the full picture — a wrong cell, a cross-language mismatch, a frame that disagrees with itself
title: "[bug] "
labels: bug
---

Use this when the short bug form is not enough: a computed frame looks wrong, one
language disagrees with another, or the behaviour depends on the data.

**What is wrong**
A clear description of the incorrect behaviour.

**The spec**

```json
# the smallest XraySpec that shows it
```

**The dataset**

```json
# the smallest recorded dataset that shows it — a handful of trades, book diffs,
# liquidations or funding points
```

**The frame you got**

```json
# the exact XrayFrame JSON
```

**The frame you expected**
Say what the right answer is and how you know: a hand computation, another tool,
or the same spec in another language.

**Does it reproduce in more than one language?**
- [ ] Rust · [ ] Python · [ ] Node.js · [ ] WASM · [ ] C · [ ] C++ · [ ] C# · [ ] Go · [ ] Java · [ ] R
- If only some: which agree and which do not?

**Does it reproduce in both builds?**
- [ ] Parallel (rayon) · [ ] Sequential (the WASM fallback)
- The two are meant to be byte-for-byte identical; a difference between them is
  itself the bug.

**Which panel**
- [ ] Footprint · [ ] Order-book heatmap · [ ] Liquidation map · [ ] Funding / OI divergence
- [ ] The frame envelope itself (cursor, symbol, bounds)

**Environment**
- `wickra-xray` version / commit:
- Rust toolchain (`rustc -V`):
- OS and architecture:

**Anything you already ruled out**
Bin or bucket size, an out-of-order event, a dataset that ends mid-bucket, a
cursor that never advanced.
