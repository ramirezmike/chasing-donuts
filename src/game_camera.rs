use crate::{player};
use bevy::prelude::*;

pub fn follow_player(
    mut cameras: Query<&mut Transform, (With<Camera3d>,  Without<player::Player>)>,
    players: Query<&Transform, (With<player::Player>, Without<Camera3d>)>,
) {
    for mut camera_transform in cameras.iter_mut() {
        for player_transform in players.iter() {
            camera_transform.translation.x = player_transform.translation.x - 1.8;
        }
    }
}

