import { createApp } from 'vue'
import init from 'wickra-xray-wasm'
import App from './App.vue'
import './style.css'

// Instantiate the WebAssembly core before mounting, so the app can construct an
// Xray synchronously. (Top-level await is enabled by vite-plugin-top-level-await.)
await init()

createApp(App).mount('#app')
