use bevy::{
    prelude::{Commands, Res, ResMut, State, Transform},
    sprite::{Anchor, Sprite, SpriteBundle},
    utils::default,
};

use crate::assets::{Manifest, Map2DAsset, TextureAssets};

use super::GameState;

pub fn swap_scene(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    texture: Res<TextureAssets>,
    manifest: Res<Manifest>,
) {
    let mut map2d_asset: Map2DAsset = Map2DAsset::new();

    for split in manifest.scene.backgrounds.clone() {
        let split_texture = texture.backgrounds[split.material_id as usize].clone();
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::TopLeft,
                ..Default::default()
            },
            texture: split_texture,
            transform: Transform::from_xyz(
                split.x_pos - 159.0,
                -split.y_pos + 145.0,
                (split.y_pos + 9700.0) / 100.0,
            ),
            ..default()
        });
    }

    for split in manifest.scene.decorates.clone() {
        let split_texture = texture.decorates[split.material_id as usize].clone();
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::TopLeft,
                    ..Default::default()
                },
                texture: split_texture.0,
                transform: Transform::from_xyz(
                    split.x_pos - split_texture.1 .0,
                    -split.y_pos + split_texture.1 .1,
                    (split.y_pos + 10000.0) / 100.0,
                ),
                ..default()
            });
            /* .with_children(|parent| {
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        anchor: Anchor::TopLeft,
                        color: Color::rgba(0.25, 0.25, 0.75, 0.5),
                        custom_size: Some(Vec2::new(
                            split_texture.2 .2 - split_texture.2 .0,
                            split_texture.2 .3 - split_texture.2 .1,
                        )),
                        ..default()
                    },
                    transform: Transform::from_xyz(split_texture.2 .0, -split_texture.2 .1, 800.0),
                    ..default()
                });
            }); */
        map2d_asset.map.insert((
            (split.x_pos - split_texture.1 .0 + split_texture.2 .0 - 2.0) as i32,
            (-split.y_pos + split_texture.1 .1 - split_texture.2 .1 + 2.0) as i32,
            (split.x_pos - split_texture.1 .0 + split_texture.2 .2 + 2.0) as i32,
            (-split.y_pos + split_texture.1 .1 - split_texture.2 .3 - 2.0) as i32,
        ));
    }

    for split in manifest.scene.beds.clone() {
        let split_texture = texture.beds[split.material_id as usize].clone();
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::TopLeft,
                    ..Default::default()
                },
                texture: split_texture.0,
                transform: Transform::from_xyz(
                    split.x_pos - split_texture.2 .0,
                    -split.y_pos + split_texture.2 .1,
                    (split.y_pos + 10000.0) / 100.0,
                ),
                ..default()
            });
            /* .with_children(|parent| {
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        anchor: Anchor::TopLeft,
                        color: Color::rgba(0.25, 0.25, 0.75, 0.5),
                        custom_size: Some(Vec2::new(
                            split_texture.3 .2 - split_texture.3 .0,
                            split_texture.3 .3 - split_texture.3 .1,
                        )),
                        ..default()
                    },
                    transform: Transform::from_xyz(split_texture.3 .0, -split_texture.3 .1, 800.0),
                    ..default()
                });
            }); */
        map2d_asset.map.insert((
            (split.x_pos - split_texture.2 .0 + split_texture.3 .0 - 2.0) as i32,
            (-split.y_pos + split_texture.2 .1 - split_texture.3 .1 + 2.0) as i32,
            (split.x_pos - split_texture.2 .0 + split_texture.3 .2 + 2.0) as i32,
            (-split.y_pos + split_texture.2 .1 - split_texture.3 .3 - 2.0) as i32,
        ));
    }

    commands.insert_resource(map2d_asset);

    state.set(GameState::Playing).unwrap();
}
