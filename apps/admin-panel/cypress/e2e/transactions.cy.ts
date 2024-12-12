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
    cy.takeScreenshot("1_click_create_button")

    cy.get('[data-testid="create-deposit-button"]').should("be.visible").click()
    cy.takeScreenshot("2_create_deposit_button")

    // Create dialog
    cy.get('[data-testid="deposit-amount-input"]')
      .type(depositAmount.toString())
      .should("have.value", depositAmount.toString())
    cy.takeScreenshot("3_enter_deposit_details")

    cy.get('[data-testid="deposit-submit-button"]').click()
    cy.takeScreenshot("4_deposit_create_submit_button")

    cy.contains("Deposit created successfully").should("be.visible")
    cy.takeScreenshot("5_deposit_created_success")
  })

  it("should show newly created Deposit in list page", () => {
    cy.visit(`/deposits`)
    cy.contains(`$${depositAmount.toLocaleString()}.00`).should("be.visible")
    cy.takeScreenshot("6_created_deposit_should_be_in_list")
  })

  it("should show newly created Deposit in customer details page", () => {
    cy.visit(`/customers/${customerId}/transactions`)
    cy.contains(`$${depositAmount.toLocaleString()}.00`).should("be.visible")
    cy.takeScreenshot("7_created_deposit_in_customer_transactions")
  })

  it("should create and cancel Withdraw", () => {
    cy.visit(`/customers/${customerId}`)
    cy.wait(1000)

    cy.get('[data-testid="global-create-button"]').click()
    cy.takeScreenshot("8_withdrawal_create_button_cancel_flow")

    cy.get('[data-testid="create-withdrawal-button"]').should("be.visible").click()
    cy.takeScreenshot("9_select_withdrawal_cancel_flow")

    cy.get('[data-testid="withdraw-amount-input"]')
      .type(withdrawAmount.toString())
      .should("have.value", withdrawAmount.toString())
    cy.takeScreenshot("10_enter_withdrawal_amount_cancel_flow")

    cy.get('[data-testid="withdraw-submit-button"]').click()
    cy.takeScreenshot("11_submit_withdrawal_cancel_flow")

    cy.get("[data-testid=withdrawal-status-badge]").then((badge) => {
      if (badge.text() === "PENDING APPROVAL") {
        // case when we have policy attached for withdrawal
        cy.get('[data-testid="approval-process-deny-button"]').click()
        cy.get('[data-testid="approval-process-dialog-deny-reason"]').type("testing")
        cy.get('[data-testid="approval-process-dialog-deny-button"]').click()
      } else {
        // expected flow
        cy.get('[data-testid="withdraw-cancel-button"]').should("be.visible").click()
        cy.takeScreenshot("12_click_cancel_button")

        cy.get('[data-testid="withdrawal-confirm-dialog-button"]')
          .should("be.visible")
          .click()
        cy.takeScreenshot("13_confirm_cancellation")

        cy.get("[data-testid=withdrawal-status-badge]")
          .should("be.visible")
          .invoke("text")
          .should("eq", "CANCELLED")
        cy.takeScreenshot("14_cancelled_status")
      }
    })
  })

  it("should create and approve Withdraw", () => {
    cy.visit(`/customers/${customerId}`)
    cy.wait(1000)

    cy.get('[data-testid="global-create-button"]').click()
    cy.takeScreenshot("15_withdrawal_create_button_approve_flow")

    cy.get('[data-testid="create-withdrawal-button"]').should("be.visible").click()
    cy.takeScreenshot("16_select_withdrawal_approve_flow")

    cy.get('[data-testid="withdraw-amount-input"]')
      .type(withdrawAmount.toString())
      .should("have.value", withdrawAmount.toString())
    cy.takeScreenshot("17_enter_withdrawal_amount_approve_flow")

    cy.get('[data-testid="withdraw-submit-button"]').click()
    cy.takeScreenshot("18_submit_withdrawal_approve_flow")

    cy.get("[data-testid=withdrawal-status-badge]")
      .then((badge) => {
        if (badge.text() === "PENDING APPROVAL") {
          cy.get('[data-testid="approval-process-approve-button"]').click()
          cy.get('[data-testid="approval-process-dialog-approve-button"]').click()
        }
      })
      .then(() => {
        cy.get('[data-testid="withdraw-confirm-button"]').should("be.visible").click()
        cy.takeScreenshot("19_click_confirm_button")

        cy.get('[data-testid="withdrawal-confirm-dialog-button"]')
          .should("be.visible")
          .click()
        cy.takeScreenshot("20_final_confirmation")

        cy.get("[data-testid=withdrawal-status-badge]")
          .should("be.visible")
          .invoke("text")
          .should("eq", "CONFIRMED")
        cy.takeScreenshot("21_confirmed_status")
      })
  })

  it("should show newly created Withdraw in list page", () => {
    cy.visit(`/withdrawals`)
    cy.contains(`$${withdrawAmount.toLocaleString()}.00`).should("be.visible")
    cy.takeScreenshot("22_newly_created_withdrawal_in_list")
  })

  it("should show newly created Withdraw in customer details page", () => {
    cy.visit(`/customers/${customerId}/transactions`)
    cy.contains(`$${withdrawAmount.toLocaleString()}.00`).should("be.visible")
    cy.takeScreenshot("23_newly_created_withdrawal_in_transactions")
  })
})
