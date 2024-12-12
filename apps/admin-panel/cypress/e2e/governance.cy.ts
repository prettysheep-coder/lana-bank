describe("Governance Test", () => {
  let committeeName: string
  let committeeId: string
  let customerId: string

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

  it("should successfully create a Committees", () => {
    committeeName = `${Date.now()}`
    cy.visit("/committees")
    cy.get('[data-testid="global-create-button"]').click()
    cy.get('[data-testid="committee-create-name-input"]')
      .type(committeeName)
      .should("have.value", committeeName)

    cy.get('[data-testid="committee-create-submit-button"]').click()
    cy.contains("Committee created successfully").should("be.visible")
    cy.getIdFromUrl("/committees/").then((id) => {
      committeeId = id
    })
  })

  it("should show newly created committee in the list", () => {
    cy.visit(`/committees/${committeeId}`)
    cy.get('[data-testid="committee-add-member-button"]').click()
    cy.get('[data-testid="committee-add-user-select"]').should("be.visible").click()
    cy.get('[role="option"]')
      .contains("admin")
      .then((option) => {
        cy.wrap(option).click()
        cy.get('[data-testid="committee-add-user-submit-button"]').click()
        cy.contains("User added to committee successfully").should("be.visible")
        cy.contains(option.text().split(" ")[0]).should("be.visible")
      })
  })

  it("should show newly added committee in the list", () => {
    cy.visit(`/committees`)
    cy.contains(committeeName).should("be.visible")
  })

  it("attach a committee to a policy", () => {
    cy.visit(`/policies`)
    cy.get('[data-testid="table-row-2"] > :nth-child(3) > a > .gap-2').click()

    cy.get('[data-testid="policy-assign-committee"]').click()

    cy.get('[data-testid="policy-select-committee-selector"]').click()
    cy.get('[role="option"]').contains(committeeName).click()
    cy.get("[data-testid=policy-assign-committee-threshold-input]").type("1")
    cy.get("[data-testid=policy-assign-committee-submit-button]").click()
  })

  it("Committee member should be able to approve a withdraw", () => {
    const amount = 1000
    cy.createDeposit(amount, customerId).then(() => {
      cy.initiateWithdrawal(amount, customerId).then((withdrawalId) => {
        cy.visit(`/actions`)
        cy.visit(`/withdrawals/${withdrawalId}`)
        cy.get("[data-testid=withdrawal-status-badge]").then((badge) => {
          if (badge.text() === "PENDING APPROVAL") {
            cy.get('[data-testid="approval-process-approve-button"]').click()
            cy.get('[data-testid="approval-process-dialog-approve-button"]').click()
          } else if (badge.text() === "PENDING CONFIRMATION") {
            throw new Error("State is Pending Confirmation")
          } else {
            throw new Error("Unexpected Withdraw State found")
          }
        })
      })
    })
  })
})
