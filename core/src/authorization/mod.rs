use std::{str::FromStr, sync::Arc};
use tokio::sync::RwLock;

pub mod error;

use error::AuthorizationError;

use crate::primitives::{Role, Subject};
use sqlx_adapter::{
    casbin::{
        prelude::{DefaultModel, Enforcer},
        CoreApi, MgmtApi,
    },
    SqlxAdapter,
};

use super::audit::Audit;

const MODEL: &str = include_str!("./rbac.conf");

#[derive(Clone)]
pub struct Authorization {
    enforcer: Arc<RwLock<Enforcer>>,
    audit: Audit,
}

impl Authorization {
    pub async fn init(pool: &sqlx::PgPool, audit: Audit) -> Result<Self, AuthorizationError> {
        let model = DefaultModel::from_str(MODEL).await?;
        let adapter = SqlxAdapter::new_with_pool(pool.clone()).await?;

        let enforcer = Enforcer::new(model, adapter).await?;

        let mut auth = Authorization {
            enforcer: Arc::new(RwLock::new(enforcer)),
            audit,
        };

        auth.seed_roles().await?;
        auth.seed_role_hierarchy().await?;

        Ok(auth)
    }

    async fn seed_roles(&mut self) -> Result<(), AuthorizationError> {
        self.add_permissions_for_superuser().await?;
        self.add_permissions_for_bank_manager().await?;
        self.add_permissions_for_admin().await?;

        Ok(())
    }

    async fn seed_role_hierarchy(&self) -> Result<(), AuthorizationError> {
        self.add_role_hierarchy(Role::Admin, Role::Superuser)
            .await?;
        self.add_role_hierarchy(Role::BankManager, Role::Admin)
            .await?;

        Ok(())
    }

    async fn add_role_hierarchy(
        &self,
        parent_role: Role,
        child_role: Role,
    ) -> Result<(), AuthorizationError> {
        let mut enforcer = self.enforcer.write().await;

        match enforcer
            .add_grouping_policy(vec![child_role.to_string(), parent_role.to_string()])
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => match AuthorizationError::from(e) {
                AuthorizationError::PermissionAlreadyExistsForRole(_) => Ok(()),
                e => Err(e),
            },
        }
    }

    async fn add_permissions_for_superuser(&mut self) -> Result<(), AuthorizationError> {
        let role = Role::Superuser;

        self.add_permission_to_role(&role, Object::User, UserAction::AssignRole(Role::Admin))
            .await?;
        self.add_permission_to_role(&role, Object::User, UserAction::RevokeRole(Role::Admin))
            .await?;
        Ok(())
    }

    async fn add_permissions_for_admin(&mut self) -> Result<(), AuthorizationError> {
        let role = Role::Admin;

        self.add_permission_to_role(&role, Object::User, UserAction::Create)
            .await?;
        self.add_permission_to_role(&role, Object::User, UserAction::List)
            .await?;
        self.add_permission_to_role(&role, Object::User, UserAction::Read)
            .await?;
        self.add_permission_to_role(&role, Object::User, UserAction::Update)
            .await?;
        self.add_permission_to_role(&role, Object::User, UserAction::Delete)
            .await?;
        self.add_permission_to_role(
            &role,
            Object::User,
            UserAction::AssignRole(Role::BankManager),
        )
        .await?;
        self.add_permission_to_role(
            &role,
            Object::User,
            UserAction::RevokeRole(Role::BankManager),
        )
        .await?;
        self.add_permission_to_role(&role, Object::Ledger, LedgerAction::Read)
            .await?;
        self.add_permission_to_role(&role, Object::Audit, AuditAction::List)
            .await?;
        Ok(())
    }

    async fn add_permissions_for_bank_manager(&mut self) -> Result<(), AuthorizationError> {
        let role = Role::BankManager;

        self.add_permission_to_role(&role, Object::Loan, LoanAction::Read)
            .await?;
        self.add_permission_to_role(&role, Object::Loan, LoanAction::List)
            .await?;
        self.add_permission_to_role(&role, Object::Loan, LoanAction::Create)
            .await?;
        self.add_permission_to_role(&role, Object::Loan, LoanAction::Approve)
            .await?;
        self.add_permission_to_role(&role, Object::Loan, LoanAction::RecordPayment)
            .await?;
        self.add_permission_to_role(&role, Object::Term, TermAction::Update)
            .await?;
        self.add_permission_to_role(&role, Object::Term, TermAction::Read)
            .await?;
        self.add_permission_to_role(&role, Object::Customer, CustomerAction::Create)
            .await?;
        self.add_permission_to_role(&role, Object::Customer, CustomerAction::List)
            .await?;
        self.add_permission_to_role(&role, Object::Customer, CustomerAction::Read)
            .await?;
        self.add_permission_to_role(&role, Object::Customer, CustomerAction::Update)
            .await?;
        self.add_permission_to_role(&role, Object::Deposit, DepositAction::Record)
            .await?;
        self.add_permission_to_role(&role, Object::Deposit, DepositAction::Read)
            .await?;
        self.add_permission_to_role(&role, Object::Deposit, DepositAction::List)
            .await?;
        self.add_permission_to_role(&role, Object::Withdraw, WithdrawAction::Initiate)
            .await?;
        self.add_permission_to_role(&role, Object::Withdraw, WithdrawAction::Confirm)
            .await?;
        self.add_permission_to_role(&role, Object::Withdraw, WithdrawAction::Read)
            .await?;
        self.add_permission_to_role(&role, Object::Withdraw, WithdrawAction::List)
            .await?;
        self.add_permission_to_role(&role, Object::Audit, AuditAction::List)
            .await?;

        Ok(())
    }

    pub async fn check_permission(
        &self,
        sub: &Subject,
        object: Object,
        action: impl Into<Action>,
    ) -> Result<bool, AuthorizationError> {
        let enforcer = self.enforcer.read().await;

        let action = action.into();
        match enforcer.enforce((sub.as_ref(), object.as_ref(), action.as_ref())) {
            Ok(true) => {
                self.audit.persist(sub, object, action, true).await?;
                Ok(true)
            }
            Ok(false) => {
                self.audit.persist(sub, object, action, false).await?;
                Err(AuthorizationError::NotAuthorized)
            }
            Err(e) => Err(AuthorizationError::Casbin(e)),
        }
    }

    pub async fn add_permission_to_role(
        &self,
        role: &Role,
        object: Object,
        action: impl Into<Action>,
    ) -> Result<(), AuthorizationError> {
        let mut enforcer = self.enforcer.write().await;

        let action = action.into();
        match enforcer
            .add_policy(vec![
                role.to_string(),
                object.to_string(),
                action.to_string(),
            ])
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => match AuthorizationError::from(e) {
                AuthorizationError::PermissionAlreadyExistsForRole(_) => Ok(()),
                e => Err(e),
            },
        }
    }

    pub async fn assign_role_to_subject(
        &self,
        sub: impl Into<Subject>,
        role: &Role,
    ) -> Result<(), AuthorizationError> {
        let sub: Subject = sub.into();
        let mut enforcer = self.enforcer.write().await;

        match enforcer
            .add_grouping_policy(vec![sub.to_string(), role.to_string()])
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => match AuthorizationError::from(e) {
                AuthorizationError::PermissionAlreadyExistsForRole(_) => Ok(()),
                e => Err(e),
            },
        }
    }

    pub async fn revoke_role_from_subject(
        &self,
        sub: impl Into<Subject>,
        role: &Role,
    ) -> Result<(), AuthorizationError> {
        let sub: Subject = sub.into();
        let mut enforcer = self.enforcer.write().await;

        match enforcer
            .remove_grouping_policy(vec![sub.to_string(), role.to_string()])
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(AuthorizationError::from(e)),
        }
    }

    pub async fn roles_for_subject(
        &self,
        sub: impl Into<Subject>,
    ) -> Result<Vec<Role>, AuthorizationError> {
        let sub: Subject = sub.into();
        let enforcer = self.enforcer.read().await;

        let roles = enforcer
            .get_grouping_policy()
            .into_iter()
            .filter(|r| r[0] == sub.to_string())
            .map(|r| Role::from(r[1].as_str()))
            .collect();

        Ok(roles)
    }
}

pub enum Object {
    Applicant,
    Loan,
    Term,
    User,
    Customer,
    Deposit,
    Withdraw,
    Audit,
    Ledger,
}

impl Object {
    const APPLICANT_STR: &'static str = "applicant";
    const LOAN_STR: &'static str = "loan";
    const TERM_STR: &'static str = "term";
    const USER_STR: &'static str = "user";
    const DEPOSIT_STR: &'static str = "deposit";
    const WITHDRAW_STR: &'static str = "withdraw";
    const CUSTOMER_STR: &'static str = "customer";
    const AUDIT_STR: &'static str = "audit";
    const LEDGER_STR: &'static str = "ledger";
}

impl AsRef<str> for Object {
    fn as_ref(&self) -> &str {
        match self {
            Object::Applicant => Self::APPLICANT_STR,
            Object::Loan => Self::LOAN_STR,
            Object::Term => Self::TERM_STR,
            Object::User => Self::USER_STR,
            Object::Deposit => Self::DEPOSIT_STR,
            Object::Withdraw => Self::WITHDRAW_STR,
            Object::Customer => Self::CUSTOMER_STR,
            Object::Audit => Self::AUDIT_STR,
            Object::Ledger => Self::LEDGER_STR,
        }
    }
}

impl std::ops::Deref for Object {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl FromStr for Object {
    type Err = crate::authorization::AuthorizationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            Self::APPLICANT_STR => Ok(Object::Applicant),
            Self::LOAN_STR => Ok(Object::Loan),
            Self::TERM_STR => Ok(Object::Term),
            Self::USER_STR => Ok(Object::User),
            Self::AUDIT_STR => Ok(Object::Audit),
            Self::CUSTOMER_STR => Ok(Object::Customer),
            Self::DEPOSIT_STR => Ok(Object::Deposit),
            Self::WITHDRAW_STR => Ok(Object::Withdraw),
            Self::LEDGER_STR => Ok(Object::Ledger),
            _ => Err(AuthorizationError::ObjectParseError {
                value: s.to_string(),
            }),
        }
    }
}

pub enum Action {
    Loan(LoanAction),
    Term(TermAction),
    User(UserAction),
    Customer(CustomerAction),
    Deposit(DepositAction),
    Withdraw(WithdrawAction),
    Audit(AuditAction),
    Ledger(LedgerAction),
}

impl AsRef<str> for Action {
    fn as_ref(&self) -> &str {
        match self {
            Action::Loan(action) => action.as_ref(),
            Action::Term(action) => action.as_ref(),
            Action::User(action) => action.as_ref(),
            Action::Customer(action) => action.as_ref(),
            Action::Deposit(action) => action.as_ref(),
            Action::Withdraw(action) => action.as_ref(),
            Action::Audit(action) => action.as_ref(),
            Action::Ledger(action) => action.as_ref(),
        }
    }
}

impl FromStr for Action {
    type Err = crate::authorization::AuthorizationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "loan-read" => Ok(Action::Loan(LoanAction::Read)),
            "loan-create" => Ok(Action::Loan(LoanAction::Create)),
            "loan-list" => Ok(Action::Loan(LoanAction::List)),
            "loan-approve" => Ok(Action::Loan(LoanAction::Approve)),
            "loan-record-payment" => Ok(Action::Loan(LoanAction::RecordPayment)),
            "term-update" => Ok(Action::Term(TermAction::Update)),
            "term-read" => Ok(Action::Term(TermAction::Read)),
            "user-create" => Ok(Action::User(UserAction::Create)),
            "user-read" => Ok(Action::User(UserAction::Read)),
            "user-list" => Ok(Action::User(UserAction::List)),
            "user-update" => Ok(Action::User(UserAction::Update)),
            "user-delete" => Ok(Action::User(UserAction::Delete)),
            "user-assign-role-superuser" => {
                Ok(Action::User(UserAction::AssignRole(Role::Superuser)))
            }
            "user-assign-role-admin" => Ok(Action::User(UserAction::AssignRole(Role::Admin))),
            "user-assign-role-bank-manager" => {
                Ok(Action::User(UserAction::AssignRole(Role::BankManager)))
            }
            "user-revoke-role-superuser" => {
                Ok(Action::User(UserAction::RevokeRole(Role::Superuser)))
            }
            "user-revoke-role-admin" => Ok(Action::User(UserAction::RevokeRole(Role::Admin))),
            "user-revoke-role-bank-manager" => {
                Ok(Action::User(UserAction::RevokeRole(Role::BankManager)))
            }
            "audit-list" => Ok(Action::Audit(AuditAction::List)),
            "customer-create" => Ok(Action::Customer(CustomerAction::Create)),
            "customer-read" => Ok(Action::Customer(CustomerAction::Read)),
            "customer-list" => Ok(Action::Customer(CustomerAction::List)),
            "customer-update" => Ok(Action::Customer(CustomerAction::Update)),
            "deposit-record" => Ok(Action::Deposit(DepositAction::Record)),
            "deposit-read" => Ok(Action::Deposit(DepositAction::Read)),
            "deposit-list" => Ok(Action::Deposit(DepositAction::List)),
            "withdraw-initiate" => Ok(Action::Withdraw(WithdrawAction::Initiate)),
            "withdraw-confirm" => Ok(Action::Withdraw(WithdrawAction::Confirm)),
            "withdraw-read" => Ok(Action::Deposit(DepositAction::Read)),
            "withdraw-list" => Ok(Action::Deposit(DepositAction::List)),
            "ledger-read" => Ok(Action::Ledger(LedgerAction::Read)),
            _ => Err(AuthorizationError::ActionParseError {
                value: s.to_string(),
            }),
        }
    }
}

impl std::ops::Deref for Action {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

pub enum LoanAction {
    List,
    Read,
    Create,
    Approve,
    RecordPayment,
}

impl LoanAction {
    const READ_STR: &'static str = "loan-read";
    const CREATE_STR: &'static str = "loan-create";
    const LIST_STR: &'static str = "loan-list";
    const APPROVE_STR: &'static str = "loan-approve";
    const RECORD_PAYMENT_STR: &'static str = "loan-record-payment";
}

impl AsRef<str> for LoanAction {
    fn as_ref(&self) -> &str {
        match self {
            LoanAction::Read => Self::READ_STR,
            LoanAction::Create => Self::CREATE_STR,
            LoanAction::List => Self::LIST_STR,
            LoanAction::Approve => Self::APPROVE_STR,
            LoanAction::RecordPayment => Self::RECORD_PAYMENT_STR,
        }
    }
}

impl std::ops::Deref for LoanAction {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl From<LoanAction> for Action {
    fn from(action: LoanAction) -> Self {
        Action::Loan(action)
    }
}

pub enum TermAction {
    Update,
    Read,
}

impl TermAction {
    const UPDATE_STR: &'static str = "term-update";
    const READ_STR: &'static str = "term-read";
}

impl AsRef<str> for TermAction {
    fn as_ref(&self) -> &str {
        match self {
            TermAction::Update => Self::UPDATE_STR,
            TermAction::Read => Self::READ_STR,
        }
    }
}

impl std::ops::Deref for TermAction {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl From<TermAction> for Action {
    fn from(action: TermAction) -> Self {
        Action::Term(action)
    }
}

pub enum AuditAction {
    List,
}

impl AuditAction {
    const LIST_STR: &'static str = "audit-list";
}

impl AsRef<str> for AuditAction {
    fn as_ref(&self) -> &str {
        match self {
            AuditAction::List => Self::LIST_STR,
        }
    }
}

impl std::ops::Deref for AuditAction {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl From<AuditAction> for Action {
    fn from(action: AuditAction) -> Self {
        Action::Audit(action)
    }
}

pub enum UserAction {
    Create,
    Read,
    List,
    Update,
    Delete,
    AssignRole(Role),
    RevokeRole(Role),
}

impl UserAction {
    const CREATE_STR: &'static str = "user-create";
    const READ_STR: &'static str = "user-read";
    const LIST_STR: &'static str = "user-list";
    const UPDATE_STR: &'static str = "user-update";
    const DELETE_STR: &'static str = "user-delete";
    const ASSIGN_ROLE_SUPERUSER_STR: &'static str = "user-assign-role-superuser";
    const ASSIGN_ROLE_ADMIN_STR: &'static str = "user-assign-role-admin";
    const ASSIGN_ROLE_BANK_MANAGER_STR: &'static str = "user-assign-role-bank-manager";
    const REVOKE_ROLE_SUPERUSER_STR: &'static str = "user-revoke-role-superuser";
    const REVOKE_ROLE_ADMIN_STR: &'static str = "user-revoke-role-admin";
    const REVOKE_ROLE_BANK_MANAGER_STR: &'static str = "user-revoke-role-bank-manager";
}

impl AsRef<str> for UserAction {
    fn as_ref(&self) -> &str {
        match self {
            UserAction::Create => Self::CREATE_STR,
            UserAction::Read => Self::READ_STR,
            UserAction::List => Self::LIST_STR,
            UserAction::Update => Self::UPDATE_STR,
            UserAction::Delete => Self::DELETE_STR,
            UserAction::AssignRole(role) => match role {
                Role::Superuser => Self::ASSIGN_ROLE_SUPERUSER_STR,
                Role::Admin => Self::ASSIGN_ROLE_ADMIN_STR,
                Role::BankManager => Self::ASSIGN_ROLE_BANK_MANAGER_STR,
            },
            UserAction::RevokeRole(role) => match role {
                Role::Superuser => Self::REVOKE_ROLE_SUPERUSER_STR,
                Role::Admin => Self::REVOKE_ROLE_ADMIN_STR,
                Role::BankManager => Self::REVOKE_ROLE_BANK_MANAGER_STR,
            },
        }
    }
}

impl std::ops::Deref for UserAction {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl From<UserAction> for Action {
    fn from(action: UserAction) -> Self {
        Action::User(action)
    }
}

pub enum CustomerAction {
    Create,
    Read,
    List,
    Update,
}

impl CustomerAction {
    const CREATE_STR: &'static str = "customer-create";
    const READ_STR: &'static str = "customer-read";
    const LIST_STR: &'static str = "customer-list";
    const UPDATE_STR: &'static str = "customer-update";
}

impl AsRef<str> for CustomerAction {
    fn as_ref(&self) -> &str {
        match self {
            CustomerAction::Create => Self::CREATE_STR,
            CustomerAction::Read => Self::READ_STR,
            CustomerAction::List => Self::LIST_STR,
            CustomerAction::Update => Self::UPDATE_STR,
        }
    }
}

impl std::ops::Deref for CustomerAction {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl From<CustomerAction> for Action {
    fn from(action: CustomerAction) -> Self {
        Action::Customer(action)
    }
}

pub enum DepositAction {
    Record,
    Read,
    List,
}

impl DepositAction {
    const RECORD_STR: &'static str = "deposit-record";
    const READ_STR: &'static str = "deposit-read";
    const LIST_STR: &'static str = "deposit-list";
}

impl AsRef<str> for DepositAction {
    fn as_ref(&self) -> &str {
        match self {
            DepositAction::Record => Self::RECORD_STR,
            DepositAction::Read => Self::READ_STR,
            DepositAction::List => Self::LIST_STR,
        }
    }
}
impl std::ops::Deref for DepositAction {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl From<DepositAction> for Action {
    fn from(action: DepositAction) -> Self {
        Action::Deposit(action)
    }
}

pub enum WithdrawAction {
    Initiate,
    Confirm,
    Read,
    List,
}

impl WithdrawAction {
    const INITIATE_STR: &'static str = "withdraw-initiate";
    const CONFIRM_STR: &'static str = "withdraw-confirm";
    const READ_STR: &'static str = "withdraw-read";
    const LIST_STR: &'static str = "withdraw-list";
}

impl AsRef<str> for WithdrawAction {
    fn as_ref(&self) -> &str {
        match self {
            WithdrawAction::Initiate => Self::INITIATE_STR,
            WithdrawAction::Confirm => Self::CONFIRM_STR,
            WithdrawAction::Read => Self::READ_STR,
            WithdrawAction::List => Self::LIST_STR,
        }
    }
}
impl std::ops::Deref for WithdrawAction {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl From<WithdrawAction> for Action {
    fn from(action: WithdrawAction) -> Self {
        Action::Withdraw(action)
    }
}

impl std::ops::Deref for LedgerAction {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

pub enum LedgerAction {
    Read,
}

impl LedgerAction {
    const READ_STR: &'static str = "ledger-read";
}

impl AsRef<str> for LedgerAction {
    fn as_ref(&self) -> &str {
        match self {
            LedgerAction::Read => Self::READ_STR,
        }
    }
}

impl From<LedgerAction> for Action {
    fn from(action: LedgerAction) -> Self {
        Action::Ledger(action)
    }
}
