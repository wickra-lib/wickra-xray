// Footprint canvas renderer (§4.5). Draws, per ascending price bin, the buy
// volume (green, right of centre) and sell volume (red, left) as diverging
// horizontal bars scaled to the panel's peak volume. Touches only the
// FootprintData model, never the core state.

import type { FootprintData } from '../lib/xray'

export function drawFootprint(
  ctx: CanvasRenderingContext2D,
  data: FootprintData,
  width: number,
  height: number,
): void {
  ctx.clearRect(0, 0, width, height)
  const n = data.price_bins.length
  if (n === 0) {
    return
  }

  const peak = Math.max(
    data.buy_vol.reduce((a, b) => Math.max(a, b), 0),
    data.sell_vol.reduce((a, b) => Math.max(a, b), 0),
    1e-9,
  )
  const rowH = height / n
  const mid = width / 2
  const half = width / 2 - 2

  for (let i = 0; i < n; i++) {
    // Bins are ascending, so higher prices are drawn towards the top.
    const y = height - (i + 1) * rowH
    const buyW = (data.buy_vol[i] / peak) * half
    const sellW = (data.sell_vol[i] / peak) * half
    ctx.fillStyle = '#3fb950'
    ctx.fillRect(mid, y + 1, buyW, Math.max(rowH - 2, 1))
    ctx.fillStyle = '#f85149'
    ctx.fillRect(mid - sellW, y + 1, sellW, Math.max(rowH - 2, 1))
  }

  ctx.strokeStyle = '#30363d'
  ctx.beginPath()
  ctx.moveTo(mid, 0)
  ctx.lineTo(mid, height)
  ctx.stroke()
}
