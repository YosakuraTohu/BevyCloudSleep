use bevy::{
    prelude::{Commands, Component, Res, Transform},
    sprite::SpriteBundle,
    utils::default,
};

use crate::assets::{Player, TextureAssets};

#[derive(Component)]
pub struct OnStagePlayer;

pub fn swap_player(mut commands: Commands, texture: Res<TextureAssets>, player: Res<Player>) {
    commands.spawn((
        OnStagePlayer,
        SpriteBundle {
            texture: texture.sleepers[player.sleeper_type].0.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 100.0,),
            ..default()
        },
    ));
}
