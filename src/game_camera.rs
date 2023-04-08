use crate::{player};
use bevy::prelude::*;

pub fn follow_player(
    mut cameras: Query<&mut Transform, (With<Camera3d>,  Without<player::Player>)>,
    players: Query<&Transform, (With<player::Player>, Without<Camera3d>)>,
    time: Res<Time>,
) {
    let camera_speed = 20.0;
    for mut camera_transform in cameras.iter_mut() {
        for player_transform in players.iter() {
            camera_transform.translation.y += 
                ((player_transform.translation.y + 0.75) - camera_transform.translation.y)
                * (camera_speed * 0.25)
                * time.delta_seconds();
            camera_transform.translation.x += 
                ((player_transform.translation.x - 2.8) - camera_transform.translation.x)
                * camera_speed
                * time.delta_seconds();
            camera_transform.translation.z += 
                (player_transform.translation.z - camera_transform.translation.z)
                * (camera_speed * 0.5)
                * time.delta_seconds();
        }
    }
}

