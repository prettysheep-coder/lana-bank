use async_graphql::dataloader::DataLoader;
use async_graphql::dataloader::Loader;

use std::collections::HashMap;

use lava_app::{app::LavaApp, user::error::UserError};

use crate::primitives::*;

use super::{committee::Committee, user::User};

pub type LavaDataLoader = DataLoader<LavaLoader>;
pub struct LavaLoader {
    pub app: LavaApp,
}

impl LavaLoader {
    pub fn new(app: &LavaApp) -> LavaDataLoader {
        DataLoader::new(Self { app: app.clone() }, tokio::task::spawn)
            // Set delay to 0 as per https://github.com/async-graphql/async-graphql/issues/1306
            .delay(std::time::Duration::from_secs(0))
    }
}

impl Loader<UserId> for LavaLoader {
    type Value = User;
    type Error = Arc<UserError>;

    async fn load(&self, keys: &[UserId]) -> Result<HashMap<UserId, User>, Self::Error> {
        self.app.users().find_all(keys).await.map_err(Arc::new)
    }
}

impl Loader<governance::CommitteeId> for LavaLoader {
    type Value = Committee;
    type Error = Arc<governance::committee_error::CommitteeError>;

    async fn load(
        &self,
        keys: &[governance::CommitteeId],
    ) -> Result<HashMap<governance::CommitteeId, Committee>, Self::Error> {
        self.app
            .governance()
            .find_all_committees(keys)
            .await
            .map_err(Arc::new)
    }
}
