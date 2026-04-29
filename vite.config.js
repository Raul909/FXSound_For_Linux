import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  test: {
    environment: "jsdom",
    setupFiles: ["./vitest.setup.js"],
  },
  plugins: [react()],
})
