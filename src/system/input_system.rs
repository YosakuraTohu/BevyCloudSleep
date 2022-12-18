use bevy::{
    input::mouse::MouseWheel,
    prelude::{
        Camera, EventReader, EventWriter, Input, MouseButton, OrthographicProjection, Query, Res,
        ResMut, Transform, Vec2, Vec3, With,
    },
    window::{CursorMoved, Windows},
};

use crate::assets::Player;

use super::{event_system::MyClickEvent, MAX_SCALE_RATE, MIN_SCALE_RATE};

pub fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut player: ResMut<Player>,
    mut my_click_event: EventWriter<MyClickEvent>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        for event in cursor_moved_events.iter() {
            let variation =
                (player.viewpot.cur_state.1 - event.position) * player.viewpot.field_of_vision;
            for mut transform in camera.iter_mut() {
                transform.translation = Vec3::new(
                    player.viewpot.position.x + variation.x,
                    player.viewpot.position.y + variation.y,
                    999.9,
                );
            }
            player.viewpot.position += variation;
            player.viewpot.cur_state.1 = event.position;
        }
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        player.viewpot.cur_state.0 = false;

        if player.viewpot.cur_start.x - 4.0 <= player.viewpot.cur_state.1.x
            && player.viewpot.cur_start.x + 4.0 >= player.viewpot.cur_state.1.x
            && player.viewpot.cur_start.y - 4.0 <= player.viewpot.cur_state.1.y
            && player.viewpot.cur_start.y + 4.0 >= player.viewpot.cur_state.1.y
        {
            let mut window_size = Vec2::default();
            for window in windows.iter() {
                window_size = Vec2::new(
                    window.physical_width() as f32,
                    window.physical_height() as f32,
                );
            }
            my_click_event.send(MyClickEvent {
                world_position: player.viewpot.position
                    + (player.viewpot.cur_start - window_size / 2.0)
                        * player.viewpot.field_of_vision,
                window_position: player.viewpot.cur_start,
                mouse_button: MouseButton::Left,
            });
        }
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        player.viewpot.cur_state.0 = true;
    }
    for event in cursor_moved_events.iter() {
        if !player.viewpot.cur_state.0 {
            player.viewpot.cur_start = event.position;
            player.viewpot.cur_state = (false, event.position);
        } else {
            player.viewpot.cur_state = (true, event.position);
        }
    }
}

pub fn mouse_wheel_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut player: ResMut<Player>,
    mut camera: Query<&mut OrthographicProjection, With<Camera>>,
) {
    if player.viewpot.field_of_vision >= MIN_SCALE_RATE
        && player.viewpot.field_of_vision <= MAX_SCALE_RATE
    {
        for event in mouse_wheel_events.iter() {
            let variation = -event.y * player.viewpot.field_of_vision / 20.0;
            player.viewpot.field_of_vision += variation;
            if player.viewpot.field_of_vision < MIN_SCALE_RATE {
                player.viewpot.field_of_vision = MIN_SCALE_RATE;
            }
            if player.viewpot.field_of_vision > MAX_SCALE_RATE {
                player.viewpot.field_of_vision = MAX_SCALE_RATE;
            }
        }

        for mut orthographic_projection in camera.iter_mut() {
            orthographic_projection.scale = player.viewpot.field_of_vision;
        }
    }
}
