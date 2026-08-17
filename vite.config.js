import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

import pkg from './package.json' with { type: 'json' }

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  define: {
    // Inject the version at build time so the status bar can never drift from
    // package.json — it was hardcoded and sat a release behind.
    __APP_VERSION__: JSON.stringify(pkg.version),
  },
})
