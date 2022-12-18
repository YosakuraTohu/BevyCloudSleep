pub mod camera_system;
pub mod event_system;
pub mod input_system;
pub mod pathfinding_system;
pub mod play_move_system;
pub mod player_system;
pub mod scene_loader_system;

pub const MIN_SCALE_RATE: f32 = 0.1;
pub const MAX_SCALE_RATE: f32 = 5.0;
pub const DEFAULT_SCALE_RATE: f32 = 1.0;

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Init,
    Loading,
    Playing,
    GameOver,
}
