import { defineConfig } from "cypress"

export default defineConfig({
  e2e: {
    baseUrl: 'http://localhost:4455',
    defaultCommandTimeout: 10000,
    requestTimeout: 10000,
    video: false,
    setupNodeEvents(on, config) {
      // implement node event listeners here
    },
  },
})
