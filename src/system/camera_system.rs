use bevy::prelude::{Camera, OrthographicProjection, Query, Res, Transform, Vec3, With};

use crate::assets::Player;

#[allow(dead_code)]
pub fn move_camera(
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    player: Res<Player>,
) {
    for (mut transform, mut orthographic_projection) in camera.iter_mut() {
        transform.translation =
            Vec3::new(player.viewpot.position.x, player.viewpot.position.y, 999.9);
        orthographic_projection.scale = player.viewpot.field_of_vision;
    }
}
