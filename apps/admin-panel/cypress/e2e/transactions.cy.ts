import { faker } from "@faker-js/faker"

describe("Transactions Deposit and Withdraw", () => {
  let customerId: string
  const depositAmount = faker.number.int({ min: 1000, max: 5000 })
  const withdrawAmount = faker.number.int({ min: 1000, max: depositAmount })

  before(() => {
    const testEmail = `test-${Date.now()}@example.com`
    const testTelegramId = `user${Date.now()}`
    cy.createCustomer(testEmail, testTelegramId).then((id) => {
      customerId = id
      cy.log(`Created customer with ID: ${id}`)
    })
  })

  beforeEach(() => {
    cy.on("uncaught:exception", (err) => {
      if (err.message.includes("ResizeObserver loop")) {
        return false
      }
    })
  })

  it("should create a Deposit", () => {
    cy.visit(`/customers/${customerId}`)
    cy.wait(1000)

    cy.get('[data-testid="global-create-button"]').click()
    cy.get('[data-testid="create-deposit-button"]').should("be.visible").click()

    // Create dialog
    cy.get('[data-testid="deposit-amount-input"]')
      .type(depositAmount.toString())
      .should("have.value", depositAmount.toString())
    cy.get('[data-testid="deposit-submit-button"]').click()
    cy.contains("Deposit created successfully").should("be.visible")
  })

  it("should show newly created Deposit in list page", () => {
    cy.visit(`/deposits`)
    cy.contains(`$${depositAmount.toLocaleString()}.00`).should("be.visible")
  })

  it("should show newly created Deposit in customer details page", () => {
    cy.visit(`/customers/${customerId}/transactions`)
    cy.contains(`$${depositAmount.toLocaleString()}.00`).should("be.visible")
  })

  it("should create and cancel Withdraw", () => {
    cy.visit(`/customers/${customerId}`)
    cy.wait(1000)

    cy.get('[data-testid="global-create-button"]').click()
    cy.get('[data-testid="create-withdrawal-button"]').should("be.visible").click()

    cy.get('[data-testid="withdraw-amount-input"]')
      .type(withdrawAmount.toString())
      .should("have.value", withdrawAmount.toString())
    cy.get('[data-testid="withdraw-submit-button"]').click()

    cy.get("[data-testid=withdrawal-status-badge]").then((badge) => {
      if (badge.text() === "PENDING APPROVAL") {
        cy.get('[data-testid="approval-process-deny-button"]').click()
        cy.get('[data-testid="approval-process-dialog-deny-reason"]').type("testing")
        cy.get('[data-testid="approval-process-dialog-deny-button"]').click()
        // TODO investigate this it is failing
      } else {
        cy.get('[data-testid="withdraw-cancel-button"]').should("be.visible").click()
        cy.get('[data-testid="withdrawal-confirm-dialog-button"]')
          .should("be.visible")
          .click()

        cy.get("[data-testid=withdrawal-status-badge]")
          .should("be.visible")
          .invoke("text")
          .should("eq", "CANCELLED")
      }
    })
  })

  it("should create and approve Withdraw", () => {
    cy.visit(`/customers/${customerId}`)
    cy.wait(1000)

    cy.get('[data-testid="global-create-button"]').click()
    cy.get('[data-testid="create-withdrawal-button"]').should("be.visible").click()

    cy.get('[data-testid="withdraw-amount-input"]')
      .type(withdrawAmount.toString())
      .should("have.value", withdrawAmount.toString())
    cy.get('[data-testid="withdraw-submit-button"]').click()

    cy.get("[data-testid=withdrawal-status-badge]")
      .then((badge) => {
        if (badge.text() === "PENDING APPROVAL") {
          cy.get('[data-testid="approval-process-approve-button"]').click()
          cy.get('[data-testid="approval-process-dialog-approve-button"]').click()
        }
      })
      .then(() => {
        cy.get('[data-testid="withdraw-confirm-button"]').should("be.visible").click()
        cy.get('[data-testid="withdrawal-confirm-dialog-button"]')
          .should("be.visible")
          .click()

        cy.get("[data-testid=withdrawal-status-badge]")
          .should("be.visible")
          .invoke("text")
          .should("eq", "CONFIRMED")
      })
  })

  it("should show newly created Withdraw in list page", () => {
    cy.visit(`/withdrawals`)
    cy.contains(`$${withdrawAmount.toLocaleString()}.00`).should("be.visible")
  })

  it("should show newly created Withdraw in customer details page", () => {
    cy.visit(`/customers/${customerId}/transactions`)
    cy.contains(`$${withdrawAmount.toLocaleString()}.00`).should("be.visible")
  })
})
