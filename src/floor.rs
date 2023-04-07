use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::{
    direction,
    player,
    AppState,
    ZeroSignum,
    CleanupMarker,
};

static FLOOR_CUBE_SIZE: f32 = 0.1;
static GROUND_SPEED: f32 = 5.0;
static NUMBER_OF_ROWS: i32 = 100;
static NUMBER_OF_COLUMNS: i32 = 30;

pub struct FloorPlugin;
impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<FloorManager>()
            .add_systems((
                    update_floors,
                    shift_floors
                )
                .in_set(OnUpdate(AppState::InGame))
            );
    }
}

#[derive(Default, Resource)]
pub struct FloorManager {
    track_distance: f32
}

pub fn spawn_floor(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    floor_manager: &mut ResMut<FloorManager>,
) { 
    let cube = meshes.add(Mesh::from(shape::Cube { size: 0.1 }));
    let columns = NUMBER_OF_COLUMNS / 2;

    floor_manager.track_distance = NUMBER_OF_ROWS as f32 * FLOOR_CUBE_SIZE;
    for x in 0..NUMBER_OF_ROWS {
        for z in -columns..columns {

//  for x in 0..1 {
//      for z in -1..1 {
            commands.spawn((
            TransformBundle::from(Transform::from_xyz(x as f32 * FLOOR_CUBE_SIZE, 0.0, z as f32 * FLOOR_CUBE_SIZE)),
            Floor { height: FLOOR_CUBE_SIZE },
            ComputedVisibility::default(),
            Visibility::Visible,
            CleanupMarker,
//          RigidBody::Fixed,
//          Collider::cuboid(0.1, 0.1, 0.1)
            )).with_children(|parent| {
                parent.spawn((
                    PbrBundle {
                        mesh: cube.clone(),
                        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                        ..default()
                    }
                        ,));
            });
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Floor {
    pub height: f32,
}

fn update_floors(
    mut commands: Commands,
    time: Res<Time>,
    mut floors: Query<(Entity, &mut Floor, &Transform, &Children)>,
    players: Query<&Transform, (With<player::Player>, Without<Floor>)>,
    mut transforms: Query<&mut Transform, (Without<player::Player>, Without<Floor>)>,
) {
    for (entity, mut floor, transform, children) in &mut floors {
        for p in &players {
            let player_translation = Vec3::new(p.translation.x, 0.0, p.translation.z);
            let floor_translation = Vec3::new(transform.translation.x, 0.0, transform.translation.z);
            let distance = (floor_translation - player_translation).length();
            if distance > 1.0 || floor_translation.x < player_translation.x {
                commands.entity(entity)
                        .remove::<RigidBody>()
                        .remove::<Collider>();
            } else {
               let half_size = FLOOR_CUBE_SIZE / 2.0;
               commands.entity(entity)
                   .insert((RigidBody::Fixed,Collider::cuboid(half_size, half_size * floor.height, half_size)));

               if distance < 0.5 {
                   floor.height += GROUND_SPEED * time.delta_seconds();
                   for child_entity in children {
                       if let Ok(mut child_transform) = transforms.get_mut(*child_entity) {
                           child_transform.scale.y = floor.height;
                       }
                   }
               }
            }
        }
    }
}

fn shift_floors(
    mut commands: Commands,
    mut floors: Query<(Entity, &Floor, &mut Transform)>,
    cameras: Query<&Transform, (With<Camera3d>, Without<Floor>)>,
    mut floor_manager: ResMut<FloorManager>,
) {
    for (entity, floor, mut transform) in &mut floors {
        for camera in &cameras {
            if transform.translation.x < camera.translation.x {
                transform.translation.x += floor_manager.track_distance;
            }
        }
    }
}
