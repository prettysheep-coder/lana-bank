describe("credit facility", () => {
  before(() => {
    Cypress.env("creditFacilityId", null)
    Cypress.env("collateralRequired", null)
  })

  it("create new terms-template", () => {
    cy.visit("/terms-templates")

    cy.get('[data-testid="global-create-button"]').click()

    const templateName = `Test Template ${Date.now()}`
    cy.get('[data-testid="terms-template-name-input"]')
      .type(templateName)
      .should("have.value", templateName)

    cy.get('[data-testid="terms-template-annual-rate-input"]')
      .type("5.5")
      .should("have.value", "5.5")

    cy.get('[data-testid="terms-template-duration-units-input"]')
      .type("12")
      .should("have.value", "12")

    cy.get('[data-testid="terms-template-duration-period-select"]').select("MONTHS")

    cy.get('[data-testid="terms-template-accrual-interval-select"]').select(
      "END_OF_MONTH",
    )
    cy.get('[data-testid="terms-template-incurrence-interval-select"]').select(
      "END_OF_MONTH",
    )

    cy.get('[data-testid="terms-template-initial-cvl-input"]')
      .type("140")
      .should("have.value", "140")

    cy.get('[data-testid="terms-template-margin-call-cvl-input"]')
      .type("120")
      .should("have.value", "120")

    cy.get('[data-testid="terms-template-liquidation-cvl-input"]')
      .type("110")
      .should("have.value", "110")

    cy.get('[data-testid="terms-template-submit-button"]').click()

    cy.url().should(
      "match",
      /\/terms-templates\/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/,
    )
    cy.contains(templateName).should("be.visible")
  })

  it("should successfully create a new credit facility using a template", () => {
    cy.visit("/customers")
    cy.contains(
      "Individuals or entities who hold accounts, loans, or credit facilities with the bank",
      { timeout: 10000 },
    )

    cy.get('[data-testid="global-create-button"]').click()
    cy.get('[data-testid="customer-create-email"]').should("be.visible")

    const testEmail = `test-${Date.now()}@example.com`
    const testTelegramId = `user${Date.now()}`

    cy.get('[data-testid="customer-create-email"]')
      .type(testEmail)
      .should("have.value", testEmail)

    cy.get('[data-testid="customer-create-telegram-id"]')
      .type(testTelegramId)
      .should("have.value", testTelegramId)

    cy.get('[data-testid="customer-create-submit-button"]')
      .contains("Review Details")
      .click()

    cy.contains(testEmail).should("be.visible")
    cy.contains(testTelegramId).should("be.visible")

    cy.get('[data-testid="customer-create-submit-button"]')
      .contains("Confirm and Submit")
      .click()

    cy.url().should(
      "match",
      /\/customers\/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/,
    )

    cy.contains(testEmail).should("be.visible")

    cy.get('[data-testid="global-create-button"]').click()
    cy.get('[data-testid="create-credit-facility-button"]').should("be.visible").click()

    cy.get('[data-testid="facility-amount-input"]')
      .type("5000")
      .should("have.value", "5000")

    cy.get('[data-testid="create-credit-facility-submit"]').click()

    cy.url()
      .should(
        "match",
        /\/credit-facilities\/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/,
      )
      .then((url) => {
        const facilityId = url.split("/").pop()
        Cypress.env("creditFacilityId", facilityId)
      })

    cy.contains("Credit Facility created successfully").should("be.visible")

    cy.get('[data-testid="collateral-to-reach-target"]')
      .should("be.visible")
      .invoke("text")
      .then((collateralValue) => {
        const numericValue = parseFloat(collateralValue.split(" ")[0])
        Cypress.env("collateralRequired", numericValue)
      })
  })

  it("should update the collateral for the created credit facility", () => {
    const creditFacilityId =
      Cypress.env("creditFacilityId") || "7b1d5da3-8771-40c7-8cea-c77a78554a8a"
    const collateralRequired = Cypress.env("collateralRequired") || 7

    expect(creditFacilityId).to.exist
    expect(collateralRequired).to.exist

    cy.visit(`/credit-facilities/${creditFacilityId}`)

    cy.wait(2000)

    cy.get('[data-testid="update-collateral-button"]').should("be.visible").click()

    cy.get('[data-testid="new-collateral-input"]')
      .should("be.visible")
      .clear()
      .type(collateralRequired.toString())

    cy.get('[data-testid="proceed-to-confirm-button"]').should("be.visible").click()
    cy.get('[data-testid="confirm-update-button"]').should("be.visible").click()
  })
})
