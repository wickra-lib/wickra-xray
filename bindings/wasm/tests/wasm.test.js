"use strict";

// The WASM binding's only test.
//
// Until now the `wasm` CI job ran `wasm-pack build --target web` and stopped.
// The binding was compiled on every push and executed on none of them, in a
// repository whose README advertises a live in-browser demo and whose core
// documents the sequential WASM path as byte-identical to the parallel one.
// Nothing checked either claim.
//
// Three checks, the same ones the other bindings make:
//
//   golden    — each committed spec, loaded with the committed dataset, must
//               return golden/expected/<spec>.json byte-for-byte, which is what
//               makes "byte-identical across ten languages" a fact rather than
//               a sentence.
//   scrubber  — frame_at(t) over the whole dataset must equal the frame over a
//               dataset that ends at t. This is the equality the scrubbing
//               claim rests on, and the golden corpus cannot see it: every
//               fixture is a full-window frame. JSON is native here, so
//               clipping the dataset is a filter rather than string surgery.
//   refusal   — an invalid spec must fail as an exception, and a command the
//               core understands but cannot carry out must answer in band.
//
// It runs under `--target nodejs`, so no browser and no headless runner are
// needed. The published artifact stays the `web` build; this one exists to be
// executed.
//
// The properties below are the ones docs/STREAMING.md states for streaming through time.

const { test } = require("node:test");
const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");

const { Xray, version } = require("../pkg-node/wickra_xray_wasm.js");

// An event timestamp in the golden dataset (1000..24000 in 1000-unit steps),
// not a gap between two: the frame over the clipped dataset takes its cursor
// from the last event it holds, so a cut inside a gap would put the two frames
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

function readJson(...parts) {
  return JSON.parse(fs.readFileSync(path.join(...parts), "utf8"));
}

function specNames() {
  return fs.readdirSync(path.join(golden, "specs")).filter((f) => f.endsWith(".json")).sort();
}

test("the module exposes its version", () => {
  assert.ok(golden, "golden fixtures not found; the checks below would test nothing");
  assert.match(version(), /^\d+\.\d+\.\d+/);
});

test("golden frames are byte-identical", () => {
  assert.ok(golden, "golden fixtures not found");
  const names = specNames();
  assert.ok(names.length > 0, "golden/specs holds no spec; this would assert nothing");
  const dataset = fs.readFileSync(path.join(golden, "data.json"), "utf8").trim();
  const load = `{"cmd":"load","dataset":${dataset}}`;

  for (const name of names) {
    const spec = fs.readFileSync(path.join(golden, "specs", name), "utf8");
    const expected = fs.readFileSync(path.join(golden, "expected", name), "utf8").trim();
    const xray = new Xray(spec);
    xray.command(load);
    const got = xray.command('{"cmd":"frame"}').trim();
    assert.strictEqual(got, expected, `${name} does not match its blessed frame`);
  }
});

test("frame_at(t) equals the frame over a dataset that ends at t", () => {
  assert.ok(golden, "golden fixtures not found");
  const spec = fs.readFileSync(path.join(golden, "specs", "multi_panel.json"), "utf8");
  const full = readJson(golden, "data.json");

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

  const scrubbed = new Xray(spec);
  scrubbed.command(`{"cmd":"load","dataset":${JSON.stringify(full)}}`);
  const atCut = scrubbed.command(`{"cmd":"frame_at","ts":${CUT}}`);

  const truncated = new Xray(spec);
  truncated.command(`{"cmd":"load","dataset":${JSON.stringify(clipped)}}`);
  const whole = truncated.command('{"cmd":"frame"}');

  assert.strictEqual(atCut, whole, "scrubbing to a timestamp is not the same as ending there");
});

test("an invalid spec raises and an unknown command answers in band", () => {
  assert.throws(() => new Xray("not json"));

  const spec = fs.readFileSync(path.join(golden, "specs", "footprint.json"), "utf8");
  const xray = new Xray(spec);
  const reply = xray.command('{"cmd":"nope"}');
  assert.match(reply, /"ok":false/);
});
