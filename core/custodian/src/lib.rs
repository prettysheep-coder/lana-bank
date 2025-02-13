mod entity;
pub mod error;
mod primitives;
mod repo;

// use audit::AuditSvc;
use authz::PermissionCheck;

pub use repo::{custodian_cursor::*, CustodianRepo};

pub struct Custodians<Perms>
where
    Perms: PermissionCheck,
{
    authz: Perms,
    repo: CustodianRepo,
}
