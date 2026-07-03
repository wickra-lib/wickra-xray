<script setup lang="ts">
// The panel grid: one card per frame panel (§4.4). This unit shows a compact
// text summary of each PanelData; the canvas renderers replace the summaries in
// §4.5, dispatched by `panel.kind`.
import type { PanelData } from '../lib/xray'

defineProps<{ panels: PanelData[] }>()

function summary(panel: PanelData): string {
  switch (panel.kind) {
    case 'footprint':
      return `${panel.price_bins.length} price bins`
    case 'book_heatmap':
      return `${panel.time.length} x ${panel.price.length} grid`
    case 'liquidation_map':
      return `${panel.events.length} liquidation events`
    case 'funding_oi_divergence':
      return `${panel.time.length} buckets`
  }
}
</script>

<template>
  <div class="grid">
    <section v-for="(panel, i) in panels" :key="i" class="panel">
      <header class="kind">{{ panel.kind }}</header>
      <p class="summary">{{ summary(panel) }}</p>
    </section>
    <p v-if="panels.length === 0" class="empty">No panels in this frame.</p>
  </div>
</template>

<style scoped>
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(15rem, 1fr));
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
  margin-bottom: 0.4rem;
}
.summary {
  margin: 0;
  color: #c9d1d9;
}
.empty {
  color: #6b7280;
}
</style>
