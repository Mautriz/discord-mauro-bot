use serenity::prelude::TypeMap;
use tokio::sync::{RwLockReadGuard, RwLockWriteGuard};

use super::context::{LupusCtx, LupusManager};
use serenity::async_trait;

#[async_trait]
pub trait LupusHelpers {
    async fn lupus(&self) -> RwLockReadGuard<LupusManager>;
    async fn lupus_mut(&self) -> RwLockWriteGuard<LupusManager>;
}

#[async_trait]
impl LupusHelpers for RwLockReadGuard<'_, TypeMap> {
    async fn lupus(&self) -> RwLockReadGuard<LupusManager> {
        self.get::<LupusCtx>().unwrap().read().await
    }

    async fn lupus_mut(&self) -> RwLockWriteGuard<LupusManager> {
        self.get::<LupusCtx>().unwrap().write().await
    }
}
