import { defineConfig } from "cypress"

export default defineConfig({
  e2e: {
    baseUrl: "http://localhost:4455",
    setupNodeEvents(on, config) {
      console.log(on, config)
    },
  },
})
