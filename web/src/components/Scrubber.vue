<script setup lang="ts">
// The time scrubber: a range slider over the dataset bounds. Moving it emits the
// new cursor timestamp, which the app folds into `frameAt(ts)` (§4.4).
import type { Bounds } from '../lib/xray'

defineProps<{ bounds: Bounds; modelValue: number }>()
const emit = defineEmits<{ 'update:modelValue': [ts: number] }>()

function onInput(event: Event): void {
  emit('update:modelValue', Number((event.target as HTMLInputElement).value))
}
</script>

<template>
  <div class="scrubber">
    <input
      type="range"
      :min="bounds.from_ts"
      :max="bounds.to_ts"
      :value="modelValue"
      step="1"
      @input="onInput"
    />
    <span class="cursor">t = {{ modelValue }}</span>
    <span class="range">[{{ bounds.from_ts }} .. {{ bounds.to_ts }}]</span>
  </div>
</template>

<style scoped>
.scrubber {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 0;
}
input[type='range'] {
  flex: 1;
  accent-color: #58a6ff;
}
.cursor {
  color: #58a6ff;
  min-width: 8rem;
}
.range {
  color: #6b7280;
  font-size: 0.8rem;
}
</style>
