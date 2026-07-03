<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { Xray } from './lib/xray'
import type { Bounds, Dataset, XrayFrame } from './lib/xray'
import { DEMO_DATASET, demoSpec, loadDatasetFromFile } from './lib/dataset'
import Scrubber from './components/Scrubber.vue'
import PanelGrid from './components/PanelGrid.vue'

// P-XRAY-4.4: wire the lib layer into the UI. The demo dataset loads on mount;
// the scrubber folds the frame up to a chosen timestamp; a JSON file can be
// uploaded to explore another recorded dataset. The X-Ray is static/historical:
// no network feed, no key. Canvas rendering of each panel arrives in §4.5.
const coreVersion = Xray.version()
const bounds = ref<Bounds | null>(null)
const frame = ref<XrayFrame | null>(null)
const cursorTs = ref(0)
const status = ref('')

let xray: Xray | null = null

function open(dataset: Dataset, label: string): void {
  if (xray) {
    xray.free()
  }
  xray = new Xray(JSON.stringify(demoSpec()))
  const loaded = xray.load(dataset)
  const b = xray.bounds()
  bounds.value = b
  cursorTs.value = b.to_ts
  frame.value = xray.frame()
  status.value = `${label}: ${loaded} events`
}

watch(cursorTs, (ts) => {
  if (xray && bounds.value) {
    frame.value = xray.frameAt(ts)
  }
})

async function onFile(event: Event): Promise<void> {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) {
    return
  }
  try {
    open(await loadDatasetFromFile(file), file.name)
  } catch (err) {
    status.value = `error: ${(err as Error).message}`
  }
}

function loadDemo(): void {
  open(DEMO_DATASET, 'demo dataset')
}

onMounted(loadDemo)
onBeforeUnmount(() => {
  if (xray) {
    xray.free()
  }
})
</script>

<template>
  <main class="app">
    <header class="header">
      <h1>Wickra X-Ray</h1>
      <span class="version">core {{ coreVersion }}</span>
    </header>

    <div class="controls">
      <label class="upload">
        Load dataset (JSON)
        <input type="file" accept="application/json,.json" @change="onFile" />
      </label>
      <button type="button" @click="loadDemo">Demo dataset</button>
      <span class="status">{{ status }}</span>
    </div>

    <Scrubber v-if="bounds" v-model="cursorTs" :bounds="bounds" />

    <p v-if="frame" class="frame-meta">
      {{ frame.symbol }} @ cursor {{ frame.cursor_ts }} — {{ frame.panels.length }} panels
    </p>
    <PanelGrid v-if="frame" :panels="frame.panels" />
  </main>
</template>

<style scoped>
.app {
  max-width: 72rem;
  margin: 0 auto;
  padding: 1.5rem;
}
.header {
  display: flex;
  align-items: baseline;
  gap: 0.75rem;
  border-bottom: 1px solid #1f2430;
  padding-bottom: 0.75rem;
}
.header h1 {
  margin: 0;
  font-size: 1.4rem;
}
.version {
  color: #58a6ff;
  font-size: 0.85rem;
}
.controls {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-top: 1rem;
  flex-wrap: wrap;
}
.upload {
  cursor: pointer;
}
button {
  background: #1f2430;
  color: #c9d1d9;
  border: 1px solid #30363d;
  border-radius: 0.4rem;
  padding: 0.4rem 0.8rem;
  cursor: pointer;
}
.status {
  color: #6b7280;
  font-size: 0.85rem;
}
.frame-meta {
  color: #8b949e;
}
</style>
