use bevy::prelude::{AssetServer, Assets, Commands, Handle, Image, Res, ResMut, State};

use crate::system::GameState;

use super::{ManiState, Manifest, TextureAssets, TextureInfo};

#[allow(dead_code)]
pub fn load_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
    manifest_assets: ResMut<Assets<Manifest>>,
    mani_state: Res<ManiState>,
) {
    let deserialized_manifest = manifest_assets.get(&mani_state.handle);
    if deserialized_manifest.is_none() {
        return;
    }
    let deserialized_manifest = deserialized_manifest.unwrap();
    //let manifest = File::open("assets/Packages/Sunset Inn/manifest.json").unwrap();
    //let deserialized_manifest: Manifest = serde_json::from_reader(manifest).unwrap();
    let TextureInfo {
        sleepers,
        backgrounds,
        decorates,
        beds,
    } = deserialized_manifest.textures.clone();
    let mut texture_assets: TextureAssets = TextureAssets::new();
    for split in sleepers {
        let mut split_assets: Vec<Handle<Image>> = Vec::new();
        for split_name in split.emote_file_names {
            split_assets
                .push(asset_server.load(format!("Packages/Sunset Inn/assets/{}", split_name)));
        }
        texture_assets.sleepers.push((
            asset_server.load(format!("Packages/Sunset Inn/assets/{}", split.file_name)),
            split_assets,
            split.offset,
        ))
    }
    for split in backgrounds {
        texture_assets
            .backgrounds
            .push(asset_server.load(format!("Packages/Sunset Inn/assets/{}", split.file_name)))
    }
    for split in decorates {
        texture_assets.decorates.push((
            asset_server.load(format!("Packages/Sunset Inn/assets/{}", split.file_name)),
            split.offset,
            split.hitbox,
        ))
    }
    for split in beds {
        let mut split_assets: Vec<Handle<Image>> = Vec::new();
        for split_name in split.sleep_file_names {
            split_assets
                .push(asset_server.load(format!("Packages/Sunset Inn/assets/{}", split_name)));
        }
        texture_assets.beds.push((
            asset_server.load(format!("Packages/Sunset Inn/assets/{}", split.file_name)),
            split_assets,
            split.offset,
            split.hitbox,
        ))
    }

    commands.insert_resource(texture_assets);
    commands.insert_resource(deserialized_manifest.clone());
    state.set(GameState::Loading).unwrap();
}
