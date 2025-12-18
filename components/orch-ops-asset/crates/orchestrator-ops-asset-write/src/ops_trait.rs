//! Asset write operations trait

use anyhow::Result;
use orchestrator_domain::Asset;
use orchestrator_ops_asset_read::AssetReadOps;

#[allow(async_fn_in_trait)]
pub trait AssetWriteOps {
    async fn create_asset(&self, asset: Asset) -> Result<Asset>;
    async fn update_asset(&self, asset: Asset) -> Result<Asset>;
    async fn delete_asset(&self, id: i64) -> Result<()>;
}

pub trait AssetOps: AssetReadOps + AssetWriteOps {}
