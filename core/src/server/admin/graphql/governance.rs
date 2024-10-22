use async_graphql::*;

use crate::{
    primitives::{ApprovalProcessType, CommitteeId},
    server::shared_graphql::{
        convert::ToGlobalId,
        primitives::{Timestamp, UUID},
    },
};

impl ToGlobalId for CommitteeId {
    fn to_global_id(&self) -> async_graphql::types::ID {
        async_graphql::types::ID::from(format!("committee:{}", self))
    }
}

#[derive(SimpleObject)]
pub struct Committee {
    id: ID,
    committee_id: UUID,
    approval_process_type: ApprovalProcessType,
    created_at: Timestamp,
}

impl From<crate::governance::Committee> for Committee {
    fn from(committee: crate::governance::Committee) -> Self {
        Self {
            id: committee.id.to_global_id(),
            committee_id: committee.id.into(),
            approval_process_type: committee.approval_process_type,
            created_at: committee.created_at().into(),
        }
    }
}

#[derive(InputObject)]
pub struct CommitteeCreateInput {
    pub approval_process_type: ApprovalProcessType,
}

#[derive(SimpleObject)]
pub struct CommitteeCreatePayload {
    pub committee: Committee,
}

impl From<crate::governance::Committee> for CommitteeCreatePayload {
    fn from(committee: crate::governance::Committee) -> Self {
        Self {
            committee: committee.into(),
        }
    }
}

#[derive(InputObject)]
pub struct CommitteeAddUserInput {
    pub user_id: UUID,
    pub approval_process_type: ApprovalProcessType,
}

#[derive(SimpleObject)]
pub struct CommitteeAddUserPayload {
    pub committee: Committee,
}

impl From<crate::governance::Committee> for CommitteeAddUserPayload {
    fn from(committee: crate::governance::Committee) -> Self {
        Self {
            committee: committee.into(),
        }
    }
}

#[derive(InputObject)]
pub struct CommitteeRemoveUserInput {
    pub user_id: UUID,
    pub approval_process_type: ApprovalProcessType,
}

#[derive(SimpleObject)]
pub struct CommitteeRemoveUserPayload {
    pub committee: Committee,
}

impl From<crate::governance::Committee> for CommitteeRemoveUserPayload {
    fn from(committee: crate::governance::Committee) -> Self {
        Self {
            committee: committee.into(),
        }
    }
}
