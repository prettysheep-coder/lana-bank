mod entity;
pub mod error;
mod primitives;
mod repo;

use audit::AuditSvc;
use authz::PermissionCheck;

use crate::primitives::ChartId;

use error::*;
pub use primitives::*;

pub struct ManualTransactions<Perms>
where
    Perms: PermissionCheck,
{
    authz: Perms,
}

impl<Perms> ManualTransactions<Perms>
where
    Perms: PermissionCheck,
{
    async fn execute(
        &self,
        sub: &<<Perms as PermissionCheck>::Audit as AuditSvc>::Subject,
        chart_id: ChartId,
        description: String,
        entries: Vec<ManualEntryInput>,
    ) -> Result<(), ManualTransactionError> {
        // check how many entries exist
        // lookup template for that amount of entries
        //   If it does not exist yet - create it
        // resolve all account ids to leaf accounts
        //   if its an account code -> lazy create the 'manual' account that backs the COA account
        //   set
        unimplemented!()
    }
}
