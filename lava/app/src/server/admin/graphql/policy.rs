use async_graphql::*;

use crate::{
    primitives::PolicyId,
    server::shared_graphql::{convert::ToGlobalId, primitives::UUID},
};

pub use governance::policy_cursor::PolicyByCreatedAtCursor;

#[derive(SimpleObject)]
pub struct Policy {
    id: ID,
    policy_id: UUID,
    process_type: String,
    committee_id: Option<UUID>,
    rules: ApprovalRules,
}

#[derive(SimpleObject)]
struct CommitteeThreshold {
    threshold: usize,
}

#[derive(SimpleObject)]
struct SystemApproval {
    auto_approve: bool,
}

#[derive(async_graphql::Union)]
enum ApprovalRules {
    CommitteeThreshold(CommitteeThreshold),
    System(SystemApproval),
}

#[derive(InputObject)]
pub struct PolicyAssignCommitteeInput {
    pub policy_id: UUID,
    pub committee_id: UUID,
    pub threshold: usize,
}

#[derive(SimpleObject)]
pub struct PolicyAssignCommitteePayload {
    policy: Policy,
}

impl ToGlobalId for PolicyId {
    fn to_global_id(&self) -> async_graphql::types::ID {
        async_graphql::types::ID::from(format!("policy:{}", self))
    }
}

impl From<governance::Policy> for Policy {
    fn from(policy: governance::Policy) -> Self {
        Self {
            id: policy.id.to_global_id(),
            policy_id: policy.id.into(),
            process_type: policy.process_type.to_string(),
            committee_id: policy.committee_id.map(UUID::from),
            rules: ApprovalRules::from(policy.rules),
        }
    }
}

impl From<governance::Policy> for PolicyAssignCommitteePayload {
    fn from(policy: governance::Policy) -> Self {
        Self {
            policy: policy.into(),
        }
    }
}

impl From<governance::ApprovalRules> for ApprovalRules {
    fn from(rules: governance::ApprovalRules) -> Self {
        match rules {
            governance::ApprovalRules::CommitteeThreshold { threshold } => {
                ApprovalRules::CommitteeThreshold(CommitteeThreshold { threshold })
            }
            governance::ApprovalRules::System => {
                ApprovalRules::System(SystemApproval { auto_approve: true })
            }
        }
    }
}
