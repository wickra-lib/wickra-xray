"use strict";

const { test } = require("node:test");
const assert = require("node:assert");
const { Xray, version } = require("../index.js");

const SPEC = JSON.stringify({
  dataset_ref: "m",
  symbol: "AAA",
  panels: [{ kind: "footprint", price_bin: 1.0, bucket_ms: 60000 }],
});

const TRADES = [
  { ts: 1000, price: 100.4, qty: 2.0, side: "buy" },
  { ts: 1400, price: 101.8, qty: 0.5, side: "buy" },
];

test("frame roundtrip returns the folded frame", () => {
  const xray = new Xray(SPEC);
  xray.command(JSON.stringify({ cmd: "load", dataset: { trades: TRADES } }));
  const frame = JSON.parse(xray.command(JSON.stringify({ cmd: "frame" })));
  assert.strictEqual(frame.symbol, "AAA");
  assert.strictEqual(frame.cursor_ts, 1400);
  assert.strictEqual(frame.panels[0].kind, "footprint");
});

test("version matches the module-level function", () => {
  const xray = new Xray(SPEC);
  assert.strictEqual(xray.version(), version());
});

test("a malformed spec throws", () => {
  assert.throws(() => new Xray("not json"));
});
