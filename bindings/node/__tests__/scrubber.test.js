"use strict";

// The scrubber equality, through the Node binding.
//
// `frame_at(t)` over the whole dataset must return exactly what `frame` returns
// over a dataset that ends at `t`. That equality is what "scrub the market like
// a video" means, and no golden fixture can check it: every blessed frame is a
// full-window frame, so the cursor never sits anywhere but the end.
//
// The two routes are genuinely different inside the core -- one clips a loaded
// window, the other never sees the later events -- so a window bound that is
// inclusive on the wrong side, a bucket that rounds outward, or a panel that
// keeps state past the cursor separates them.

const { test } = require("node:test");
const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const { Xray } = require("../index.js");

// An event timestamp in the golden dataset (1000..24000 in 1000-unit steps),
// not a gap between two: the frame over the clipped dataset takes its cursor
// from the last event it holds, so a cut inside a gap would leave the two frames
// on different cursors and compare nothing.
const CUT = 12000;

function findGolden() {
  let dir = __dirname;
  for (let i = 0; i < 8; i++) {
    const g = path.join(dir, "golden");
    if (fs.existsSync(path.join(g, "specs"))) {
      return g;
    }
    dir = path.dirname(dir);
  }
  return null;
}

const golden = findGolden();

function spec() {
  return fs.readFileSync(path.join(golden, "specs", "multi_panel.json"), "utf8");
}

function dataset() {
  return JSON.parse(fs.readFileSync(path.join(golden, "data.json"), "utf8"));
}

function loaded(data) {
  const xray = new Xray(spec());
  xray.command(JSON.stringify({ cmd: "load", dataset: data }));
  return xray;
}

test("frame_at(t) equals the frame over a dataset that ends at t", () => {
  assert.ok(golden, "golden fixtures not found; this would test nothing");
  const full = dataset();

  const clipped = {};
  let kept = 0;
  let dropped = 0;
  for (const [stream, events] of Object.entries(full)) {
    clipped[stream] = events.filter((e) => {
      if (e.ts <= CUT) {
        kept++;
        return true;
      }
      dropped++;
      return false;
    });
  }
  // A cut that keeps everything or nothing would compare a frame with itself.
  assert.ok(kept > 0 && dropped > 0, `the cut at ${CUT} kept ${kept} and dropped ${dropped}`);

  const atCut = loaded(full).command(JSON.stringify({ cmd: "frame_at", ts: CUT }));
  const whole = loaded(clipped).command(JSON.stringify({ cmd: "frame" }));

  assert.strictEqual(atCut, whole, "scrubbing to a timestamp is not the same as ending there");
});

test("folding to the end reproduces the full frame", () => {
  assert.ok(golden, "golden fixtures not found");
  // The upper bound is where an off-by-one hides: the spec leaves `to_ts` open,
  // so `frame`'s cursor is the dataset's own end.
  const full = dataset();
  const end = Math.max(...Object.values(full).flat().map((e) => e.ts));
  const xray = loaded(full);
  assert.strictEqual(
    xray.command(JSON.stringify({ cmd: "frame_at", ts: end })),
    xray.command(JSON.stringify({ cmd: "frame" })),
  );
});

test("a midpoint frame reports its own cursor", () => {
  assert.ok(golden, "golden fixtures not found");
  const frame = JSON.parse(loaded(dataset()).command(JSON.stringify({ cmd: "frame_at", ts: CUT })));
  assert.strictEqual(frame.cursor_ts, CUT);
});
