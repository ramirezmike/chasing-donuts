use bevy::prelude::*;
use crate::{floor, CleanupMarker, FOOD_COLOR, random_in_f32_range, player, audio, assets};
use std::f32::consts::TAU;

pub struct FoodPlugin;
impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnFoodEvent>();
    }
}

#[derive(Component, Reflect, Default, Clone, Copy)]
pub struct Food {
}

pub struct SpawnFoodEvent;


pub fn update_food(
    mut commands: Commands,
    mut foods: Query<(Entity, &Food, &mut Transform), Without<player::Player>>,
    mut floor_manager: ResMut<floor::FloorManager>,
    player: Query<&Transform, With<player::Player>>,
    time: Res<Time>,
    mut audio: audio::GameAudio,
    game_assets: Res<assets::GameAssets>,
) {
    for p in &player {
        for (entity, _, mut food_transform) in &mut foods {
            food_transform.rotate_y(time.delta_seconds() * 1.2);
            food_transform.scale = Vec3::splat(1.0 + (time.elapsed_seconds().sin().abs() * 0.2));

            if p.translation.distance(food_transform.translation) < 1.0 {
                audio.play_sfx(&game_assets.collect);
                floor_manager.score += 10;
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn spawn_food(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnFoodEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    floor_manager: Res<floor::FloorManager>,
) {
    for _ in event_reader.iter() {
        let (closest_left, farthest_right) = floor_manager.current_level_size();
        let x = random_in_f32_range(closest_left.x, farthest_right.x);
        let z = random_in_f32_range(closest_left.y, farthest_right.y);
        let (_, highest) = floor_manager.current_level_heights();

        commands
            .spawn((
                CleanupMarker,
                Food {},
                PbrBundle {
                    mesh: meshes.add(Mesh::from(
                      shape::Torus { 
                          radius: 0.5,
                          ring_radius: 0.25,
                          subdivisions_segments: 8,
                          subdivisions_sides: 6,
                      }
                    )),
                    material: materials.add(Color::hex(FOOD_COLOR).unwrap().into()),
                    transform: {
                        let mut t = Transform::from_xyz(x, highest + 0.5, z);
                        t.rotate_z(TAU * 0.25);
                        t
                    },
                    ..default()
                }
            ));
    }
}
