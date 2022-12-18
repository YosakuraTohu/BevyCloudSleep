use bevy::prelude::{EventReader, Query, Res, ResMut, Transform, Vec2, With};
use pathfinding::prelude::astar;

use crate::assets::{Map2DAsset, PlayerPathRes};

use super::{event_system::MyClickEvent, player_system::OnStagePlayer};

pub fn on_event_move_in_pathfinding(
    mut event: EventReader<MyClickEvent>,
    mut player_path: ResMut<PlayerPathRes>,
    player: Query<&Transform, With<OnStagePlayer>>,
    map2d: Res<Map2DAsset>,
) {
    for my_click_event in event.iter() {
        //info!("{:#?}", my_click_event);
        for position in player.iter() {
            let start = (
                position.translation.x as i32,
                position.translation.y as i32 - 40,
            );
            let target = (
                my_click_event.world_position.x as i32,
                my_click_event.world_position.y as i32,
            );
            if map2d.map.is_in_reat((&target.0, &target.1)) {
                //info!("true");
                return;
            }

            let result = astar(
                &start,
                |&(x, y)| {
                    if map2d.map.is_on_edge((&x, &y)) {
                        return vec![];
                    }
                    vec![
                        ((x - 15, y + 15), 14),
                        ((x, y + 15), 10),
                        ((x + 15, y + 15), 14),
                        ((x - 15, y), 10),
                        ((x + 15, y), 10),
                        ((x - 15, y - 15), 14),
                        ((x, y - 15), 10),
                        ((x + 15, y - 15), 14),
                    ]
                },
                |&(x, y)| (target.0.abs_diff(x) + target.1.abs_diff(y)) * 10,
                |&p| {
                    target.0 - 16 <= p.0
                        && target.0 + 16 >= p.0
                        && target.1 - 16 <= p.1
                        && target.1 + 16 >= p.1
                },
            );

            let mut message = PlayerPathRes::new();
            message.is_active = true;
            if let Some(r) = result {
                for (x, y) in r.0 {
                    message.path.push(Vec2::new(x as f32, y as f32));
                }
                message.number_of_times = r.1;
            }

            player_path.is_active = message.is_active;
            player_path.path = message.path;
            player_path.number_of_times = message.number_of_times;
        }
    }
}
