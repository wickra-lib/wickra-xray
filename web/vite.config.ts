import { fileURLToPath } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import wasm from 'vite-plugin-wasm'

// The xray core is a real WebAssembly module (bindings/wasm, built with
// wasm-pack). The wasm plugin lets Vite bundle and instantiate it
// client-side, so the browser renderer runs the exact same core as the CLI.
// The module's async `init()` uses top-level await, which the esnext build
// target supports natively -- vite-plugin-top-level-await is not needed for
// that and does not run under Vite 8's Rolldown build.
//
// The module is imported as `wickra-xray-wasm` through an alias onto the
// wasm-pack output rather than a `file:` dependency: that output is built, not
// committed, so a `file:` entry in package.json is a path Dependabot cannot
// read, and it failed every update of this directory. tsconfig.json maps the
// same name to the generated typings.
export default defineConfig({
  base: '/',
  plugins: [vue(), wasm()],
  resolve: {
    alias: {
      'wickra-xray-wasm': fileURLToPath(
        new URL('../bindings/wasm/pkg/wickra_xray_wasm.js', import.meta.url),
      ),
    },
  },
  build: { target: 'esnext' },
})
