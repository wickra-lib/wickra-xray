// Book-heatmap canvas renderer (§4.5). Draws the dense [T][P] resting-liquidity
// matrix as a grid: columns are time buckets, rows are price bins (ascending
// upward), each cell shaded by its intensity on a dark-to-blue ramp. Touches
// only the HeatmapData model.

import type { HeatmapData } from '../lib/xray'

export function drawHeatmap(
  ctx: CanvasRenderingContext2D,
  data: HeatmapData,
  width: number,
  height: number,
): void {
  ctx.clearRect(0, 0, width, height)
  const cols = data.time.length
  const rows = data.price.length
  if (cols === 0 || rows === 0) {
    return
  }

  let peak = 1e-9
  for (const column of data.intensity) {
    for (const value of column) {
      if (value > peak) {
        peak = value
      }
    }
  }

  const cellW = width / cols
  const cellH = height / rows
  for (let t = 0; t < cols; t++) {
    const column = data.intensity[t]
    for (let p = 0; p < rows; p++) {
      const v = column[p] / peak
      // Price ascending, so row 0 sits at the bottom.
      const y = height - (p + 1) * cellH
      ctx.fillStyle = `rgb(${Math.round(v * 88)}, ${Math.round(v * 166)}, ${Math.round(v * 255)})`
      ctx.fillRect(t * cellW, y, cellW + 0.5, cellH + 0.5)
    }
  }
}
