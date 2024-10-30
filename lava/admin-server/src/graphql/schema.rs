use async_graphql::{types::connection::*, Context, Object};

use lava_app::app::LavaApp;

use crate::primitives::*;

use super::{
    approval_process::*, audit::*, authenticated_subject::*, committee::*, customer::*,
    document::*, loader::*, policy::*, user::*,
};

pub struct Query;

#[Object]
impl Query {
    async fn me(&self, ctx: &Context<'_>) -> async_graphql::Result<AuthenticatedSubject> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        let user = Arc::new(app.users().find_for_subject(sub).await?);
        let loader = ctx.data_unchecked::<LavaDataLoader>();
        loader.feed_one(user.id, User::from(user.clone())).await;
        Ok(AuthenticatedSubject::from(user))
    }

    async fn user(&self, ctx: &Context<'_>, id: UUID) -> async_graphql::Result<Option<User>> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        maybe_fetch_one!(User, ctx, app.users().find_by_id(sub, id))
    }

    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        let loader = ctx.data_unchecked::<LavaDataLoader>();
        let users: Vec<_> = app
            .users()
            .list_users(sub)
            .await?
            .into_iter()
            .map(User::from)
            .collect();
        loader
            .feed_many(users.iter().map(|u| (u.entity.id, u.clone())))
            .await;
        Ok(users)
    }

    async fn customer(
        &self,
        ctx: &Context<'_>,
        id: UUID,
    ) -> async_graphql::Result<Option<Customer>> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        maybe_fetch_one!(Customer, ctx, app.customers().find_by_id(sub, id))
    }

    async fn customer_by_email(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> async_graphql::Result<Option<Customer>> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        maybe_fetch_one!(Customer, ctx, app.customers().find_by_email(sub, email))
    }

    async fn customers(
        &self,
        ctx: &Context<'_>,
        first: i32,
        after: Option<String>,
    ) -> async_graphql::Result<Connection<CustomerByEmailCursor, Customer, EmptyFields, EmptyFields>>
    {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        list_with_cursor!(
            CustomerByEmailCursor,
            Customer,
            ctx,
            after,
            first,
            |query| app.customers().list(sub, query)
        )
    }

    async fn committee(
        &self,
        ctx: &Context<'_>,
        id: UUID,
    ) -> async_graphql::Result<Option<Committee>> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        maybe_fetch_one!(
            Committee,
            ctx,
            app.governance().find_committee_by_id(sub, id)
        )
    }

    async fn committees(
        &self,
        ctx: &Context<'_>,
        first: i32,
        after: Option<String>,
    ) -> async_graphql::Result<
        Connection<CommitteeByCreatedAtCursor, Committee, EmptyFields, EmptyFields>,
    > {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        list_with_cursor!(
            CommitteeByCreatedAtCursor,
            Committee,
            ctx,
            after,
            first,
            |query| app.governance().list_committees(sub, query)
        )
    }

    async fn policy(&self, ctx: &Context<'_>, id: UUID) -> async_graphql::Result<Option<Policy>> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        maybe_fetch_one!(Policy, ctx, app.governance().find_policy(sub, id))
    }

    async fn policies(
        &self,
        ctx: &Context<'_>,
        first: i32,
        after: Option<String>,
    ) -> async_graphql::Result<Connection<PolicyByCreatedAtCursor, Policy, EmptyFields, EmptyFields>>
    {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        list_with_cursor!(
            PolicyByCreatedAtCursor,
            Policy,
            ctx,
            after,
            first,
            |query| app.governance().list_policies_by_created_at(sub, query)
        )
    }

    async fn approval_process(
        &self,
        ctx: &Context<'_>,
        id: UUID,
    ) -> async_graphql::Result<Option<ApprovalProcess>> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        maybe_fetch_one!(
            ApprovalProcess,
            ctx,
            app.governance().find_approval_process_by_id(sub, id)
        )
    }

    async fn approval_processes(
        &self,
        ctx: &Context<'_>,
        first: i32,
        after: Option<String>,
    ) -> async_graphql::Result<
        Connection<ApprovalProcessByCreatedAtCursor, ApprovalProcess, EmptyFields, EmptyFields>,
    > {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        list_with_cursor!(
            ApprovalProcessByCreatedAtCursor,
            ApprovalProcess,
            ctx,
            after,
            first,
            |query| app.governance().list_approval_processes(sub, query)
        )
    }

    async fn document(
        &self,
        ctx: &Context<'_>,
        id: UUID,
    ) -> async_graphql::Result<Option<Document>> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        maybe_fetch_one!(Document, ctx, app.documents().find_by_id(sub, id))
    }

    async fn audit(
        &self,
        ctx: &Context<'_>,
        first: i32,
        after: Option<String>,
    ) -> async_graphql::Result<Connection<AuditCursor, AuditEntry>> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        query(
            after,
            None,
            Some(first),
            None,
            |after, _, first, _| async move {
                let first = first.expect("First always exists");
                let res = app
                    .list_audit(
                        sub,
                        es_entity::PaginatedQueryArgs {
                            first,
                            after: after.map(lava_app::audit::AuditCursor::from),
                        },
                    )
                    .await?;

                let mut connection = Connection::new(false, res.has_next_page);
                connection
                    .edges
                    .extend(res.entities.into_iter().map(|entry| {
                        let cursor = AuditCursor::from(&entry);
                        Edge::new(cursor, AuditEntry::from(entry))
                    }));

                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    pub async fn customer_document_attach(
        &self,
        ctx: &Context<'_>,
        input: DocumentCreateInput,
    ) -> async_graphql::Result<DocumentCreatePayload> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        let file = input.file.value(ctx)?;
        exec_mutation!(
            DocumentCreatePayload,
            Document,
            ctx,
            app.documents()
                .create(sub, file.content.to_vec(), input.customer_id, file.filename)
        )
    }

    async fn user_create(
        &self,
        ctx: &Context<'_>,
        input: UserCreateInput,
    ) -> async_graphql::Result<UserCreatePayload> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        exec_mutation!(
            UserCreatePayload,
            User,
            ctx,
            app.users().create_user(sub, input.email)
        )
    }

    async fn user_assign_role(
        &self,
        ctx: &Context<'_>,
        input: UserAssignRoleInput,
    ) -> async_graphql::Result<UserAssignRolePayload> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        let UserAssignRoleInput { id, role } = input;
        exec_mutation!(
            UserAssignRolePayload,
            User,
            ctx,
            app.users().assign_role_to_user(sub, id, role)
        )
    }

    async fn user_revoke_role(
        &self,
        ctx: &Context<'_>,
        input: UserRevokeRoleInput,
    ) -> async_graphql::Result<UserRevokeRolePayload> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        let UserRevokeRoleInput { id, role } = input;
        exec_mutation!(
            UserRevokeRolePayload,
            User,
            ctx,
            app.users().revoke_role_from_user(sub, id, role)
        )
    }

    async fn committee_create(
        &self,
        ctx: &Context<'_>,
        input: CommitteeCreateInput,
    ) -> async_graphql::Result<CommitteeCreatePayload> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        exec_mutation!(
            CommitteeCreatePayload,
            Committee,
            ctx,
            app.governance().create_committee(sub, input.name)
        )
    }

    async fn committee_add_user(
        &self,
        ctx: &Context<'_>,
        input: CommitteeAddUserInput,
    ) -> async_graphql::Result<CommitteeAddUserPayload> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        exec_mutation!(
            CommitteeAddUserPayload,
            Committee,
            ctx,
            app.governance()
                .add_user_to_committee(sub, input.committee_id, input.user_id)
        )
    }

    async fn committee_remove_user(
        &self,
        ctx: &Context<'_>,
        input: CommitteeRemoveUserInput,
    ) -> async_graphql::Result<CommitteeRemoveUserPayload> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        exec_mutation!(
            CommitteeRemoveUserPayload,
            Committee,
            ctx,
            app.governance()
                .remove_user_from_committee(sub, input.committee_id, input.user_id)
        )
    }

    async fn policy_assign_committee(
        &self,
        ctx: &Context<'_>,
        input: PolicyAssignCommitteeInput,
    ) -> async_graphql::Result<PolicyAssignCommitteePayload> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        exec_mutation!(
            PolicyAssignCommitteePayload,
            Policy,
            ctx,
            app.governance().assign_committee_to_policy(
                sub,
                input.policy_id,
                input.committee_id,
                input.threshold
            )
        )
    }

    async fn approval_process_approve(
        &self,
        ctx: &Context<'_>,
        input: ApprovalProcessApproveInput,
    ) -> async_graphql::Result<ApprovalProcessApprovePayload> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        exec_mutation!(
            ApprovalProcessApprovePayload,
            ApprovalProcess,
            ctx,
            app.governance().approve_process(sub, input.process_id)
        )
    }

    async fn approval_process_deny(
        &self,
        ctx: &Context<'_>,
        input: ApprovalProcessDenyInput,
    ) -> async_graphql::Result<ApprovalProcessDenyPayload> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        exec_mutation!(
            ApprovalProcessDenyPayload,
            ApprovalProcess,
            ctx,
            app.governance().deny_process(sub, input.process_id)
        )
    }

    async fn document_download_link_generate(
        &self,
        ctx: &Context<'_>,
        input: DocumentDownloadLinksGenerateInput,
    ) -> async_graphql::Result<DocumentDownloadLinksGeneratePayload> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        // not using macro here because DocumentDownloadLinksGeneratePayload is non standard
        let doc = app
            .documents()
            .generate_download_link(sub, input.document_id.into())
            .await?;
        Ok(DocumentDownloadLinksGeneratePayload::from(doc))
    }

    async fn document_delete(
        &self,
        ctx: &Context<'_>,
        input: DocumentDeleteInput,
    ) -> async_graphql::Result<DocumentDeletePayload> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        // not using macro here because DocumentDeletePayload is non standard
        app.documents()
            .delete(sub, input.document_id.clone())
            .await?;
        Ok(DocumentDeletePayload {
            deleted_document_id: input.document_id,
        })
    }

    async fn document_archive(
        &self,
        ctx: &Context<'_>,
        input: DocumentArchiveInput,
    ) -> async_graphql::Result<DocumentArchivePayload> {
        let (app, sub) = app_and_sub_from_ctx!(ctx);
        exec_mutation!(
            DocumentArchivePayload,
            Document,
            ctx,
            app.documents().archive(sub, input.document_id)
        )
    }
}
