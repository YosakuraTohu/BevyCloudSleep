use bevy::{
    prelude::{Component, Handle, Image, Resource, Vec2},
    reflect::TypeUuid,
};
use serde::{Deserialize, Serialize};

use crate::{system::DEFAULT_SCALE_RATE, utils::map2d::Map2D};

pub mod catch_assets_manifest;
pub mod custom_asset;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Describe {
    pub ipport: String,
    pub mainclient: String,
    pub mainclient_howtoget: String,
    pub compatible_clients: String,
    pub description: String,
    pub guid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneSpritePos {
    pub material_id: f32,
    pub x_pos: f32,
    pub y_pos: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scene {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub default_background: f32,
    pub sleepers: Vec<SceneSpritePos>,
    pub backgrounds: Vec<SceneSpritePos>,
    pub decorates: Vec<SceneSpritePos>,
    pub beds: Vec<SceneSpritePos>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SleepersTextureInfo {
    pub file_name: String,
    pub offset: (f32, f32),
    pub emote_file_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackgroundsTextureInfo {
    pub file_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecoratesTextureInfo {
    pub file_name: String,
    pub offset: (f32, f32),
    pub hitbox: (f32, f32, f32, f32),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BedsTextureInfo {
    pub file_name: String,
    pub offset: (f32, f32),
    pub hitbox: (f32, f32, f32, f32),
    pub sleep_file_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextureInfo {
    pub sleepers: Vec<SleepersTextureInfo>,
    pub backgrounds: Vec<BackgroundsTextureInfo>,
    pub decorates: Vec<DecoratesTextureInfo>,
    pub beds: Vec<BedsTextureInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Resource, TypeUuid)]
#[uuid = "b6afd355-a6b1-47ef-89bd-54981bc5777f"]
pub struct Manifest {
    pub package_name: String,
    pub describe: Describe,
    pub scene: Scene,
    pub textbox_place_holders: Vec<String>,
    pub textures: TextureInfo,
}

#[allow(dead_code)]
#[derive(Clone, Component, Resource)]
pub struct TextureAssets {
    pub sleepers: Vec<(Handle<Image>, Vec<Handle<Image>>, (f32, f32))>,
    pub backgrounds: Vec<Handle<Image>>,
    pub decorates: Vec<(Handle<Image>, (f32, f32), (f32, f32, f32, f32))>,
    pub beds: Vec<(
        Handle<Image>,
        Vec<Handle<Image>>,
        (f32, f32),
        (f32, f32, f32, f32),
    )>,
}

#[allow(dead_code)]
impl TextureAssets {
    pub fn new() -> Self {
        TextureAssets {
            sleepers: Vec::new(),
            backgrounds: Vec::new(),
            decorates: Vec::new(),
            beds: Vec::new(),
        }
    }
}

#[derive(Clone, Component, Resource)]
pub struct Viewport {
    pub cur_start: Vec2,
    pub cur_state: (bool, Vec2),
    pub position: Vec2,
    pub field_of_vision: f32,
}

#[allow(dead_code)]
#[derive(Clone, Component, Resource)]
pub struct Player {
    pub viewpot: Viewport,
    pub sleeper_type: usize,
}

#[allow(dead_code)]
impl Player {
    pub fn new() -> Self {
        Player {
            viewpot: Viewport {
                cur_start: Vec2 { x: 0.0, y: 0.0 },
                cur_state: (false, Vec2 { x: 0.0, y: 0.0 }),
                position: Vec2 { x: 0.0, y: 0.0 },
                field_of_vision: DEFAULT_SCALE_RATE,
            },
            sleeper_type: 0,
        }
    }
}

#[derive(Resource, Default)]
pub struct ManiState {
    pub handle: Handle<Manifest>,
}

#[derive(Resource, Default)]
pub struct Map2DAsset {
    pub map: Map2D,
}

impl Map2DAsset {
    pub fn new() -> Self {
        Map2DAsset { map: Map2D::new() }
    }
}

#[derive(Resource, Default)]
pub struct PlayerPathRes {
    pub is_active: bool,
    pub path: Vec<Vec2>,
    pub number_of_times: u32,
    pub last_point: Vec2,
    pub cur: u8,
}

impl PlayerPathRes {
    pub fn new() -> Self {
        PlayerPathRes {
            is_active: false,
            path: Vec::default(),
            number_of_times: 0,
            last_point: Vec2::default(),
            cur: 0,
        }
    }
}
