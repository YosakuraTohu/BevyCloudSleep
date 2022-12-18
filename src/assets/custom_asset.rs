use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    utils::BoxedFuture,
};

use super::Manifest;

#[derive(Default)]
pub struct ManifestLoader;

impl AssetLoader for ManifestLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let manifest_asset = serde_json::from_slice::<Manifest>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(manifest_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
