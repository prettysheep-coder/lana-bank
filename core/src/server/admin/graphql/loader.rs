use async_graphql::dataloader::Loader;

use super::user::User;
use crate::{app::LavaApp, primitives::UserId, user::error::UserError};

use std::{collections::HashMap, sync::Arc};

pub struct LavaDataLoader {
    pub app: LavaApp,
}

impl Loader<UserId> for LavaDataLoader {
    type Value = User;
    type Error = Arc<UserError>;

    async fn load(&self, keys: &[UserId]) -> Result<HashMap<UserId, User>, Self::Error> {
        self.app.users().find_all(keys).await.map_err(Arc::new)
    }
}
