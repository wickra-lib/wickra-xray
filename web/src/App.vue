<script setup lang="ts">
import { ref } from 'vue'
import { version } from 'wickra-xray-wasm'

// P-XRAY-4.1 scaffold: the WASM core is instantiated (main.ts) and its version
// read here to prove the binding loads. The dataset loader, the time scrubber
// and the four panel canvases arrive in P-XRAY-4.2..4.5.
const coreVersion = ref(version())
</script>

<template>
  <main class="app">
    <header class="header">
      <h1>Wickra X-Ray</h1>
      <span class="version">core {{ coreVersion }}</span>
    </header>
    <p class="hint">
      Market-microstructure explorer. Load a recorded dataset and scrub through
      the footprint, order-book heatmap, liquidation map and funding/OI
      divergence panels — all rendered from the core's <code>XrayFrame</code>.
    </p>
    <p class="hint muted">Panels and the scrubber arrive in P-XRAY-4.2+.</p>
  </main>
</template>

<style scoped>
.app {
  max-width: 60rem;
  margin: 0 auto;
  padding: 2rem 1.5rem;
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
.hint {
  line-height: 1.6;
}
.muted {
  color: #6b7280;
}
code {
  color: #58a6ff;
}
</style>
