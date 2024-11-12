// ***********************************************************
// This example support/e2e.ts is processed and
// loaded automatically before your test files.
//
// This is a great place to put global configuration and
// behavior that modifies Cypress.
//
// You can change the location of this file or turn off
// automatically serving support files with the
// 'supportFile' configuration option.
//
// You can read more here:
// https://on.cypress.io/configuration
// ***********************************************************

// Import commands.js using ES2015 syntax:
// eslint-disable-next-line import/no-unassigned-import
import "./commands"

// Alternatively you can use CommonJS syntax:
// require('./commands')

const DEV_AUTH_CONFIG = {
  nextAuthUrl: "http://localhost:4455/admin-panel/api/auth",
  mailhogUrl: "http://localhost:8025",
  defaultEmail: "admin@galoy.io",
  callbackUrl: "/admin-panel/profile",
} as const

const AUTH_CONFIG = {
  nextAuthUrl: "http://admin.staging.lava.galoy.io/api/auth",
  mailhogUrl: "",
  defaultEmail: "galoysuperuser@mailinator.com",
  callbackUrl: "/profile",
} as const

export const E2E_CONFIG =
  process.env.NODE_ENV === "development" ? DEV_AUTH_CONFIG : AUTH_CONFIG
