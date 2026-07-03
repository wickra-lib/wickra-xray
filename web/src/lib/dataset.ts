// Dataset loading for the web renderer (§4.3).
//
// The X-Ray is a historical/static explorer: there is no network feed and no API
// key. A dataset is either a JSON file the user uploads or the bundled demo
// below. The core validates the streams on `load` and surfaces any problem
// in-band, so this layer only parses and type-asserts the shape.

import type { Dataset, XraySpec } from './xray'

/** A small deterministic demo dataset exercising all four panels. */
export const DEMO_DATASET: Dataset = {
  candles: [
    { ts: 1000, open: 100.0, high: 100.6, low: 99.9, close: 100.4, volume: 12 },
    { ts: 3000, open: 100.4, high: 101.5, low: 100.1, close: 101.2, volume: 18 },
    { ts: 5000, open: 101.2, high: 101.9, low: 100.8, close: 101.0, volume: 9 },
  ],
  trades: [
    { ts: 1000, price: 100.4, qty: 2.0, side: 'buy' },
    { ts: 1100, price: 100.9, qty: 1.0, side: 'sell' },
    { ts: 1200, price: 101.2, qty: 3.0, side: 'buy' },
    { ts: 2000, price: 100.1, qty: 1.5, side: 'sell' },
    { ts: 3000, price: 101.8, qty: 0.5, side: 'buy' },
    { ts: 4000, price: 100.7, qty: 2.5, side: 'sell' },
  ],
  book: [
    {
      ts: 1000,
      kind: 'snapshot',
      bids: [[100.0, 3.0], [99.5, 5.0]],
      asks: [[100.5, 2.0], [101.0, 4.0]],
    },
    {
      ts: 3000,
      kind: 'diff',
      bids: [[100.0, 1.0], [99.5, 0.0]],
      asks: [[100.5, 3.0]],
    },
  ],
  funding: [
    { ts: 1000, rate: 0.0001 },
    { ts: 3000, rate: 0.0002 },
    { ts: 5000, rate: 0.00015 },
  ],
  oi: [
    { ts: 1000, oi: 500 },
    { ts: 2000, oi: 600 },
    { ts: 4000, oi: 550 },
  ],
  liquidations: [
    { ts: 1500, price: 100.5, qty: 2.0, side: 'long' },
    { ts: 2500, price: 101.0, qty: 1.0, side: 'short' },
    { ts: 3500, price: 99.5, qty: 3.0, side: 'long' },
  ],
}

/** A spec pairing with the demo dataset: one of each panel kind. */
export function demoSpec(): XraySpec {
  return {
    dataset_ref: 'demo',
    symbol: 'DEMO',
    panels: [
      { kind: 'footprint', price_bin: 1.0, bucket_ms: 60000 },
      { kind: 'book_heatmap', price_bin: 0.5, bucket_ms: 1000, depth_levels: 8 },
      { kind: 'liquidation_map', price_bin: 1.0 },
      { kind: 'funding_oi_divergence', bucket_ms: 1000 },
    ],
  }
}

/** Parse a `Dataset` from an uploaded JSON file. */
export async function loadDatasetFromFile(file: File): Promise<Dataset> {
  const text = await file.text()
  let parsed: unknown
  try {
    parsed = JSON.parse(text)
  } catch {
    throw new Error('dataset file is not valid JSON')
  }
  if (parsed === null || typeof parsed !== 'object' || Array.isArray(parsed)) {
    throw new Error('dataset must be a JSON object of event streams')
  }
  // The core validates each stream on `load`; trust the shape here.
  return parsed as Dataset
}
