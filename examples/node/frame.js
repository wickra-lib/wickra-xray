// A runnable Node.js example: build a frame through the binding.
//
//   ( cd bindings/node && npm install && npm run build )
//   ( cd examples/node && npm install && node frame.js )

"use strict";

const { Xray, version } = require("wickra-xray");

const SPEC = JSON.stringify({
  dataset_ref: "m",
  symbol: "AAA",
  panels: [{ kind: "footprint", price_bin: 1.0, bucket_ms: 60000 }],
});

const DATASET = {
  trades: [
    { ts: 1000, price: 100.4, qty: 2.0, side: "buy" },
    { ts: 1400, price: 101.8, qty: 0.5, side: "sell" },
  ],
};

const xray = new Xray(SPEC);
xray.command(JSON.stringify({ cmd: "load", dataset: DATASET }));
const response = xray.command(JSON.stringify({ cmd: "frame" }));
const frame = JSON.parse(response);

console.log("wickra-xray", version());
console.log(response);
console.log(`  panels: ${frame.panels.length}`);
