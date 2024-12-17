# Users

Follow these steps to manage users in the application.

---

### Step 1: Visit the Users Page

Navigate to the Users page where you can see a data table listing all users with their email addresses and roles.

![Step 1: Users List](./screenshots/user.cy.ts/1_users_list.png)

---

<!-- new-page -->

### Step 2: Create a New User

Click on the "Create" button to open the user creation dialog.

![Step 2: Click Create Button](./screenshots/user.cy.ts/2_click_create_button.png)

---

<!-- new-page -->

### Step 3: Enter User Email

In the dialog form, input the email address for the new user. This will be used to send them a magic link for access.

![Step 3: Enter Email](./screenshots/user.cy.ts/3_enter_email.png)

---

<!-- new-page -->

### Step 4: Assign Admin Role

Use the checkbox to assign roles. This determines the user's initial permissions in the system, you can update these in future

![Step 4: Assign Admin Role](./screenshots/user.cy.ts/4_assign_admin_role.png)

---

<!-- new-page -->

### Step 5: Submit User Creation

Click the submit button. This will:
- Create the user account
- Generate a magic link
- Send the link to the provided email

![Step 5: Submit Creation](./screenshots/user.cy.ts/5_submit_creation.png)

---

<!-- new-page -->

### Step 6: Verify User Creation

A success message will appear confirming that:
- The user has been created
- A magic link has been sent

![Step 6: Verify Creation](./screenshots/user.cy.ts/6_verify_creation.png)

---

<!-- new-page -->

### Step 7: View User in List

The data table will update to show the newly created user with their email and assigned roles.

![Step 7: View in List](./screenshots/user.cy.ts/7_view_in_list.png)

---

<!-- new-page -->

### Step 8: Manage User Roles

Click on a user to view their details. Use the roles dropdown menu to modify their permissions.

![Step 8: Manage Roles](./screenshots/user.cy.ts/8_manage_roles.png)

---

<!-- new-page -->

### Step 9: Update Roles

In the roles dropdown:
- Check a role to assign it
- Uncheck a role to revoke it
Available roles include Admin, Accountant, and others as configured.

![Step 9: Update Roles](./screenshots/user.cy.ts/9_update_roles.png)

---

<!-- new-page -->

### Step 10: Verify Role Update

The user's details will update to reflect the new role assignments.

![Step 10: Verify Update](./screenshots/user.cy.ts/10_verify_update.png)
