use async_graphql::*;

use lava_app::{app::LavaApp, authorization::VisibleNavigationItems, user::User as DomainUser};

use crate::primitives::*;

pub struct AuthenticatedSubject {
    entity: Arc<DomainUser>,
}

#[Object]
#[graphql(name = "Subject")]
impl AuthenticatedSubject {
    async fn user(&self) -> super::user::User {
        Arc::clone(&self.entity).into()
    }

    async fn visible_navigation_items(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<VisibleNavigationItems> {
        let app = ctx.data_unchecked::<LavaApp>();
        let AdminAuthContext { sub } = ctx.data()?;
        let permissions = app.get_visible_nav_items(sub).await?;
        Ok(permissions)
    }

    async fn subject_create_customer(&self, ctx: &Context<'_>) -> async_graphql::Result<bool> {
        let app = ctx.data_unchecked::<LavaApp>();
        let AdminAuthContext { sub } = ctx.data()?;
        Ok(app
            .customers()
            .subject_can_create_customer(sub, false)
            .await
            .is_ok())
    }

    async fn subject_create_user(&self, ctx: &Context<'_>) -> async_graphql::Result<bool> {
        let app = ctx.data_unchecked::<LavaApp>();
        let AdminAuthContext { sub } = ctx.data()?;
        Ok(app.users().subject_create_user(sub, false).await.is_ok())
    }

    async fn subject_can_create_terms_template(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<bool> {
        let app = ctx.data_unchecked::<LavaApp>();
        let AdminAuthContext { sub } = ctx.data()?;
        Ok(app
            .terms_templates()
            .subject_can_create_terms_template(sub, false)
            .await
            .is_ok())
    }
}

impl From<Arc<DomainUser>> for AuthenticatedSubject {
    fn from(entity: Arc<DomainUser>) -> Self {
        Self { entity }
    }
}
