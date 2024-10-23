use async_graphql::{dataloader::DataLoader, *};

use crate::{
    app::LavaApp,
    primitives::{ApprovalProcessType, CommitteeId, ProcessAssignmentId, UserId},
    server::shared_graphql::{
        convert::ToGlobalId,
        primitives::{Timestamp, UUID},
    },
};

use super::{user::User, LavaDataLoader};

impl ToGlobalId for CommitteeId {
    fn to_global_id(&self) -> async_graphql::types::ID {
        async_graphql::types::ID::from(format!("committee:{}", self))
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Committee {
    id: ID,
    committee_id: UUID,
    #[graphql(skip)]
    user_ids: Vec<UUID>,
    created_at: Timestamp,
}

#[ComplexObject]
impl Committee {
    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let loader = ctx.data_unchecked::<DataLoader<LavaDataLoader>>();
        let users = loader
            .load_many(self.user_ids.iter().map(UserId::from))
            .await?
            .into_values()
            .map(User::from)
            .collect();

        Ok(users)
    }
}

impl From<crate::governance::Committee> for Committee {
    fn from(committee: crate::governance::Committee) -> Self {
        Self {
            id: committee.id.to_global_id(),
            committee_id: committee.id.into(),
            user_ids: committee.users().iter().map(|user| user.into()).collect(),
            created_at: committee.created_at().into(),
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct ProcessAssignment {
    id: ID,
    process_assignment_id: UUID,
    #[graphql(skip)]
    committee_id: Option<UUID>,
    approval_process_type: ApprovalProcessType,
}

#[ComplexObject]
impl ProcessAssignment {
    async fn committee(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Committee>> {
        if let Some(id) = &self.committee_id {
            let app = ctx.data_unchecked::<LavaApp>();
            let committee = app
                .governance()
                .find_committee_by_id_internal(id.into())
                .await?;
            return Ok(Some(committee.into()));
        }
        Ok(None)
    }
}

impl ToGlobalId for ProcessAssignmentId {
    fn to_global_id(&self) -> async_graphql::types::ID {
        async_graphql::types::ID::from(format!("process_assignment:{}", self))
    }
}

impl From<crate::governance::ProcessAssignment> for ProcessAssignment {
    fn from(process_assignment: crate::governance::ProcessAssignment) -> Self {
        Self {
            id: process_assignment.id.to_global_id(),
            process_assignment_id: process_assignment.id.into(),
            committee_id: process_assignment.committee_id.map(Into::into),
            approval_process_type: process_assignment.approval_process_type,
        }
    }
}

#[derive(InputObject)]
pub struct CommitteeUpdateInput {
    pub process_assignment_id: UUID,
    pub committee_id: UUID,
}

#[derive(SimpleObject)]
pub struct CommitteeUpdatePayload {
    pub process_assignment: ProcessAssignment,
}

impl From<crate::governance::ProcessAssignment> for CommitteeUpdatePayload {
    fn from(process_assignment: crate::governance::ProcessAssignment) -> Self {
        Self {
            process_assignment: process_assignment.into(),
        }
    }
}

#[derive(InputObject)]
pub struct CommitteeCreateInput {
    pub name: String,
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
    pub committee_id: UUID,
    pub user_id: UUID,
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
    pub committee_id: UUID,
    pub user_id: UUID,
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
