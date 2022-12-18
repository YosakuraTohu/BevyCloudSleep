use bevy::prelude::{Query, ResMut, Transform, With};

use crate::assets::PlayerPathRes;

use super::player_system::OnStagePlayer;

#[allow(dead_code)]
pub fn on_event_move(
    mut player: Query<&mut Transform, With<OnStagePlayer>>,
    mut player_path: ResMut<PlayerPathRes>,
) {
    if player_path.is_active {
        if player_path.path.len() <= 2 {
            player_path.is_active = false;
            return;
        }

        if player_path.cur == 0 {
            player_path.cur = 10;
            player_path.last_point = player_path.path.remove(0);
        }

        for mut position in player.iter_mut() {
            if let Some(path) = player_path.path.first() {
                position.translation.x = -(path.x - player_path.last_point.x) * (player_path.cur as f32 / 10.0) + path.x;
                position.translation.y = -(path.y - player_path.last_point.y) * (player_path.cur as f32 / 10.0) + path.y + 40.0;
                position.translation.z = (-path.y + 10000.0) / 100.0;
            }
            player_path.cur-=1;
        }
    }
}
