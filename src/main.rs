mod assets;
mod system;
mod utils;

use assets::{
    catch_assets_manifest::load_scene, custom_asset::ManifestLoader, ManiState, Manifest, Player,
    PlayerPathRes,
};
use bevy::{prelude::*, time::FixedTimestep};
use system::{
    event_system::MyClickEvent,
    input_system::{mouse_click_system, mouse_wheel_system},
    pathfinding_system::on_event_move_in_pathfinding,
    play_move_system::on_event_move,
    player_system::swap_player,
    scene_loader_system::swap_scene,
    GameState,
};

const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<ManiState>()
        .add_asset::<Manifest>()
        .init_asset_loader::<ManifestLoader>()
        .insert_resource(PlayerPathRes::new())
        .add_event::<MyClickEvent>()
        .insert_resource(ClearColor(Color::Rgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }))
        .add_state(GameState::Init)
        .add_startup_system(startup)
        .add_system_set(SystemSet::on_update(GameState::Init).with_system(load_scene))
        .add_system_set(
            SystemSet::on_update(GameState::Loading)
                .with_system(swap_scene)
                .with_system(swap_player),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(mouse_click_system)
                .with_system(mouse_wheel_system)
                .with_system(on_event_move_in_pathfinding),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(on_event_move),
        )
        .run();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut mani_state: ResMut<ManiState>,
) {
    mani_state.handle = asset_server.load("Packages/Sunset Inn/manifest.json");

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 999.9),
        ..default()
    });
    commands.insert_resource(Player::new());
}
