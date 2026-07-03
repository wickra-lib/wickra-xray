<script setup lang="ts">
// The panel grid: one canvas per frame panel (§4.4/4.5). Each PanelData is
// dispatched by `kind` to its canvas renderer, which draws the render
// data-model directly (never the core state).
import { nextTick, onMounted, watch } from 'vue'
import type { PanelData } from '../lib/xray'
import { drawFootprint } from '../render/footprint'
import { drawHeatmap } from '../render/heatmap'
import { drawLiqMap } from '../render/liquidation_map'
import { drawDivergence } from '../render/funding_oi_divergence'

const props = defineProps<{ panels: PanelData[] }>()

const CANVAS_HEIGHT = 200
const canvases: Array<HTMLCanvasElement | null> = []

function setCanvas(index: number, el: unknown): void {
  canvases[index] = el as HTMLCanvasElement | null
}

function draw(): void {
  props.panels.forEach((panel, i) => {
    const el = canvases[i]
    if (!el) {
      return
    }
    const width = el.clientWidth || 300
    const height = CANVAS_HEIGHT
    el.width = width
    el.height = height
    const ctx = el.getContext('2d')
    if (!ctx) {
      return
    }
    switch (panel.kind) {
      case 'footprint':
        drawFootprint(ctx, panel, width, height)
        break
      case 'book_heatmap':
        drawHeatmap(ctx, panel, width, height)
        break
      case 'liquidation_map':
        drawLiqMap(ctx, panel, width, height)
        break
      case 'funding_oi_divergence':
        drawDivergence(ctx, panel, width, height)
        break
    }
  })
}

onMounted(() => {
  void nextTick(draw)
})
watch(
  () => props.panels,
  () => {
    void nextTick(draw)
  },
  { flush: 'post' },
)
</script>

<template>
  <div class="grid">
    <section v-for="(panel, i) in panels" :key="i" class="panel">
      <header class="kind">{{ panel.kind }}</header>
      <canvas :ref="(el) => setCanvas(i, el)" class="canvas"></canvas>
    </section>
    <p v-if="panels.length === 0" class="empty">No panels in this frame.</p>
  </div>
</template>

<style scoped>
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(20rem, 1fr));
  gap: 1rem;
  margin-top: 1rem;
}
.panel {
  border: 1px solid #1f2430;
  border-radius: 0.5rem;
  padding: 0.9rem 1rem;
  background: #0f131b;
}
.kind {
  color: #58a6ff;
  font-weight: 600;
  margin-bottom: 0.5rem;
}
.canvas {
  display: block;
  width: 100%;
  height: 200px;
  background: #0b0e14;
  border-radius: 0.3rem;
}
.empty {
  color: #6b7280;
}
</style>
