---
name: Feature request (detailed)
about: A new panel kind, spec field, dataset input or binding surface
title: "[feat] "
labels: enhancement
---

**The view you cannot build today**
Describe what you want to see in words first, then show how far the current spec
gets you.

```json
# the closest XraySpec you can write now, and where it falls short
```

**What you would want to write**

```json
# the spec you wish worked
```

**What the frame should carry**
X-Ray returns render data-models, never renderer commands: the core decides what
a panel *means*, the front-end decides how it looks. Say what new numbers the
`XrayFrame` would carry — not how they should be drawn.

**Why it belongs in the core rather than around it**
A frame is data, not code, so anything added here has to cross the C ABI and
WASM unchanged and mean the same thing in ten languages. Say why this cannot be
a step the caller takes before or after the frame is built.

**Does it need a new dataset input?**
- [ ] No — it is computable from trades, book diffs, liquidations and funding/OI
- [ ] Yes (say which, and where a recorded feed for it comes from)

**Effect on existing specs**
- [ ] Additive: existing specs and golden frames are untouched
- [ ] Changes an existing meaning (say which, and why that is right)

**Alternatives you considered**
