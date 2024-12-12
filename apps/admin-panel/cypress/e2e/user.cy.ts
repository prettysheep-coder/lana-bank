describe("Users", () => {
  let userEmail: string
  let userId: string

  beforeEach(() => {
    cy.on("uncaught:exception", (err) => {
      if (err.message.includes("ResizeObserver loop")) {
        return false
      }
    })
  })

  it("should create a user successfully", () => {
    userEmail = `test-${Date.now()}@example.com`

    cy.visit(`/users`)
    cy.wait(1000)
    cy.get('[data-testid="global-create-button"]').click()

    cy.get('[data-testid="create-user-email-input"]')
      .type(userEmail)
      .should("have.value", userEmail)

    cy.get('[data-testid="create-user-role-admin-checkbox"]').click()

    cy.get('[data-testid="create-user-submit-button"]').click()
    cy.contains("Magic link sent successfully").should("be.visible")

    cy.get("[data-testid=user-details-email]")
      .should("be.visible")
      .invoke("text")
      .should("eq", userEmail)

    cy.url().then((url) => {
      userId = url.split("/users/")[1]
    })
  })

  it("should show newly created user in the list", () => {
    cy.visit("/users")
    cy.wait(1000)
    cy.contains(userEmail).should("be.visible")
  })

  it("Can update user roles", () => {
    cy.visit(`/users/${userId}`)
    cy.wait(1000)
    cy.get('[data-testid="user-details-manage-role"]').click()
    cy.get('[data-testid="user-details-manage-role-accountant-checkbox"]').click()
    cy.contains("Role assigned").should("be.visible")
  })
})
