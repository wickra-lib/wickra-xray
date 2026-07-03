// Funding/OI-divergence canvas renderer (§4.5). Draws the three index-aligned
// series (price, funding, open interest) as stacked mini line charts over the
// shared time axis, each normalised to its own lane so their divergence is
// legible. Touches only the DivergenceData model.

import type { DivergenceData } from '../lib/xray'

export function drawDivergence(
  ctx: CanvasRenderingContext2D,
  data: DivergenceData,
  width: number,
  height: number,
): void {
  ctx.clearRect(0, 0, width, height)
  const n = data.time.length
  if (n === 0) {
    return
  }

  const series: Array<{ name: string; values: number[]; color: string }> = [
    { name: 'price', values: data.price, color: '#58a6ff' },
    { name: 'funding', values: data.funding, color: '#3fb950' },
    { name: 'oi', values: data.oi, color: '#d29922' },
  ]
  const laneH = height / series.length

  series.forEach((s, lane) => {
    const top = lane * laneH
    let min = Infinity
    let max = -Infinity
    for (const v of s.values) {
      if (v < min) {
        min = v
      }
      if (v > max) {
        max = v
      }
    }
    const span = Math.max(max - min, 1e-9)

    ctx.strokeStyle = s.color
    ctx.lineWidth = 1.5
    ctx.beginPath()
    for (let i = 0; i < n; i++) {
      const x = n === 1 ? width / 2 : (i / (n - 1)) * (width - 8) + 4
      const y = top + laneH - 4 - ((s.values[i] - min) / span) * (laneH - 8)
      if (i === 0) {
        ctx.moveTo(x, y)
      } else {
        ctx.lineTo(x, y)
      }
    }
    ctx.stroke()

    ctx.fillStyle = s.color
    ctx.font = '10px monospace'
    ctx.fillText(s.name, 4, top + 12)
  })
}
