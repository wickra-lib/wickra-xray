// Liquidation-map canvas renderer (§4.5). Plots each liquidation as a marker at
// (time, price bin): the x-axis is time, the y-axis price (higher upward), the
// radius scales with liquidated quantity, and the colour is the side (green for
// long, red for short). Touches only the LiqMapData model.

import type { LiqMapData } from '../lib/xray'

export function drawLiqMap(
  ctx: CanvasRenderingContext2D,
  data: LiqMapData,
  width: number,
  height: number,
): void {
  ctx.clearRect(0, 0, width, height)
  const events = data.events
  if (events.length === 0) {
    return
  }

  let tMin = Infinity
  let tMax = -Infinity
  let pMin = Infinity
  let pMax = -Infinity
  let qMax = 1e-9
  for (const e of events) {
    tMin = Math.min(tMin, e.ts)
    tMax = Math.max(tMax, e.ts)
    pMin = Math.min(pMin, e.price_bin)
    pMax = Math.max(pMax, e.price_bin)
    qMax = Math.max(qMax, e.qty)
  }
  const tSpan = Math.max(tMax - tMin, 1)
  const pSpan = Math.max(pMax - pMin, 1)
  const pad = 8

  for (const e of events) {
    const x = ((e.ts - tMin) / tSpan) * (width - 2 * pad) + pad
    const y = height - (((e.price_bin - pMin) / pSpan) * (height - 2 * pad) + pad)
    const r = 3 + (e.qty / qMax) * 10
    ctx.fillStyle = e.side === 'long' ? 'rgba(63, 185, 80, 0.7)' : 'rgba(248, 81, 73, 0.7)'
    ctx.beginPath()
    ctx.arc(x, y, r, 0, Math.PI * 2)
    ctx.fill()
  }
}
