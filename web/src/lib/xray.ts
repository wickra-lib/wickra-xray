// Typed wrapper over the WASM `Xray` binding (§4.2).
//
// The core speaks a single JSON command protocol (§6.6): every method here
// serialises a `{cmd, ...}` envelope, calls `command`, and parses the response.
// Domain errors come back in-band as `{"ok":false,"error":...}`, which this
// wrapper turns into a thrown `Error`. The render data-model types mirror the
// core's `XrayFrame` (§6.4) exactly, so the panel renderers are fully typed.

import { Xray as WasmXray, version as coreVersion } from 'wickra-xray-wasm'

// --- input data model (the recorded dataset, §6.1) ---

export type Side = 'buy' | 'sell'
export type LiqSide = 'long' | 'short'
export type BookKind = 'snapshot' | 'diff'

export interface Trade {
  ts: number
  price: number
  qty: number
  side: Side
}

export interface BookEvent {
  ts: number
  kind: BookKind
  bids: Array<[number, number]>
  asks: Array<[number, number]>
}

export interface FundingEvent {
  ts: number
  rate: number
}

export interface OiEvent {
  ts: number
  oi: number
}

export interface LiquidationEvent {
  ts: number
  price: number
  qty: number
  side: LiqSide
}

export interface Candle {
  ts: number
  open: number
  high: number
  low: number
  close: number
  volume: number
}

export interface Dataset {
  candles?: Candle[]
  trades?: Trade[]
  book?: BookEvent[]
  funding?: FundingEvent[]
  oi?: OiEvent[]
  liquidations?: LiquidationEvent[]
}

// --- spec (§6.2 / §6.3) ---

export type XrayPanel =
  | { kind: 'footprint'; price_bin?: number; bucket_ms?: number }
  | { kind: 'book_heatmap'; price_bin?: number; bucket_ms?: number; depth_levels?: number }
  | { kind: 'liquidation_map'; price_bin?: number }
  | { kind: 'funding_oi_divergence'; bucket_ms?: number }

export interface XraySpec {
  dataset_ref: string
  symbol: string
  from_ts?: number
  to_ts?: number
  panels: XrayPanel[]
}

// --- render data-models (the output frame, §6.4) ---

export interface FootprintData {
  price_bins: number[]
  buy_vol: number[]
  sell_vol: number[]
}

export interface HeatmapData {
  time: number[]
  price: number[]
  intensity: number[][]
}

export interface LiqEvent {
  ts: number
  price_bin: number
  qty: number
  side: LiqSide
}

export interface LiqMapData {
  events: LiqEvent[]
}

export interface DivergenceData {
  time: number[]
  funding: number[]
  oi: number[]
  price: number[]
}

export type PanelData =
  | ({ kind: 'footprint' } & FootprintData)
  | ({ kind: 'book_heatmap' } & HeatmapData)
  | ({ kind: 'liquidation_map' } & LiqMapData)
  | ({ kind: 'funding_oi_divergence' } & DivergenceData)

export interface XrayFrame {
  symbol: string
  cursor_ts: number
  panels: PanelData[]
}

export interface Bounds {
  from_ts: number
  to_ts: number
  count: number
}

interface ErrorResponse {
  ok?: boolean
  error?: string
}

/** A typed handle over the WASM core, driven by JSON commands. */
export class Xray {
  private inner: WasmXray

  /** Build an xray from a spec JSON string (`''` for an empty handle). */
  constructor(specJson = '') {
    this.inner = new WasmXray(specJson)
  }

  /** The library version. */
  static version(): string {
    return coreVersion()
  }

  /** Replace the spec. */
  setSpec(spec: XraySpec): void {
    this.command({ cmd: 'set_spec', spec })
  }

  /** Load a dataset; returns the number of events loaded. */
  load(dataset: Dataset): number {
    return this.command<{ loaded: number }>({ cmd: 'load', dataset }).loaded
  }

  /** The full-window frame (`cursor_ts = to_ts`). */
  frame(): XrayFrame {
    return this.command<XrayFrame>({ cmd: 'frame' })
  }

  /** The frame folded up to `ts` (the scrubber path). */
  frameAt(ts: number): XrayFrame {
    return this.command<XrayFrame>({ cmd: 'frame_at', ts })
  }

  /** The dataset bounds, for the scrubber. */
  bounds(): Bounds {
    return this.command<Bounds>({ cmd: 'bounds' })
  }

  /** Clear the dataset, keeping the spec. */
  reset(): void {
    this.command({ cmd: 'reset' })
  }

  /** Free the underlying WASM instance. */
  free(): void {
    this.inner.free()
  }

  /** Serialise a command, apply it, and parse the response (throwing on an
   * in-band `{"ok":false,...}` error). */
  private command<T = unknown>(envelope: unknown): T {
    const parsed = JSON.parse(this.inner.command(JSON.stringify(envelope))) as T & ErrorResponse
    if (parsed !== null && typeof parsed === 'object' && parsed.ok === false) {
      throw new Error(parsed.error ?? 'wickra-xray: command failed')
    }
    return parsed
  }
}
