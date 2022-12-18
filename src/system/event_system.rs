use bevy::prelude::{MouseButton, Vec2};

#[derive(Debug)]
pub struct MyClickEvent {
    pub world_position: Vec2,
    pub window_position: Vec2,
    pub mouse_button: MouseButton,
}
