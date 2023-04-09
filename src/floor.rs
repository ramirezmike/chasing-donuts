use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::{
    direction,
    food,
    player,
    AppState,
    ZeroSignum,
    CleanupMarker,
    random_number,
    FLOOR_COLOR,
};
use std::collections::{HashMap, VecDeque};

static FLOOR_CUBE_SIZE: f32 = 0.3;
static GROUND_SPEED: f32 = 20.0;
static NUMBER_OF_LIVE_ROWS: i32 = 100;
static NUMBER_OF_ROWS: i32 = 200;
static NUMBER_OF_COLUMNS: i32 = 30;
static DISTANCE_INCREASE: f32 = 0.1;


pub struct FloorPlugin;
impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<FloorManager>();
    }
}

#[derive(Default, Resource)]
pub struct FloorManager {
    pub track_distance: i32,
    pub title_screen_cooldown: f32,
    pub score: usize,
    floor_rows: VecDeque::<FloorRow>,
    lowest: f32,
    highest: f32,
    cube_mesh: Handle<Mesh>,
}

impl FloorManager {
    pub fn current_level_size(&self) -> (Vec2, Vec2) {
        let farthest_visible_x = self.track_distance as f32 * FLOOR_CUBE_SIZE;
        let start_x = farthest_visible_x - (NUMBER_OF_LIVE_ROWS as f32 * FLOOR_CUBE_SIZE);
        let end_x = start_x + (NUMBER_OF_ROWS as f32 * FLOOR_CUBE_SIZE);
        let half_width = (NUMBER_OF_COLUMNS as f32 * FLOOR_CUBE_SIZE) / 2.0;

        (Vec2::new(start_x, -half_width), Vec2::new(end_x, half_width))
    }

    pub fn current_level_heights(&self) -> (f32, f32) {
        (self.lowest * FLOOR_CUBE_SIZE, self.highest * FLOOR_CUBE_SIZE)
    }
}

#[derive(Default)]
pub struct FloorRow {
    blocks: VecDeque::<Floor>,
}

#[derive(Component, Reflect, Default, Clone, Copy)]
#[reflect(Component)]
pub struct Floor {
    pub height: f32,
    pub row_id: usize,
    pub z: f32,
    pub color: Color,
}


pub fn setup_floor(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    floor_manager: &mut ResMut<FloorManager>,
    food_spawn_event_writer: &mut EventWriter<food::SpawnFoodEvent>,
) { 
    **floor_manager = FloorManager::default();
    let columns = NUMBER_OF_COLUMNS / 2;
    floor_manager.cube_mesh = meshes.add(Mesh::from(shape::Cube { size: FLOOR_CUBE_SIZE }));
    floor_manager.lowest = 2.0; 

    let mut row_id: usize = 0;
    for _ in 0..NUMBER_OF_ROWS {
        let mut floor_row = FloorRow::default();
        for z in -columns..columns {
            let c = Color::hex(FLOOR_COLOR).unwrap();// * Vec3::new(color_x, color_x, color_x);

            floor_row.blocks.push_front( Floor { 
                height: 2.0, 
                z: z as f32, 
                row_id, 
                color: c 
            });
        }

        row_id += 1;
        floor_manager.floor_rows.push_front(floor_row);
    }

    spawn_floors(commands, meshes, materials, NUMBER_OF_LIVE_ROWS, floor_manager, food_spawn_event_writer);
}

pub fn update_floors(
    mut commands: Commands,
    time: Res<Time>,
    mut floors: Query<(Entity, &mut Floor, &Transform, &Children)>,
    players: Query<(&Transform, &Velocity, &player::Player), Without<Floor>>,
    mut transforms: Query<&mut Transform, (Without<player::Player>, Without<Floor>)>,
) {
    for (p, p_velocity, player) in &players {
        if p_velocity.linvel.x < player.speed * 0.1 {
            return;
        }

        for (entity, mut floor, transform, children) in &mut floors {
            let player_translation = Vec3::new(p.translation.x, 0.0, p.translation.z);
            let floor_translation = Vec3::new(transform.translation.x, 0.0, transform.translation.z);
            let distance = (floor_translation - player_translation).length();
            if distance > 2.0 || floor_translation.x <= player_translation.x {
                commands.entity(entity)
                        .remove::<RigidBody>()
                        .remove::<Collider>();
            } 
            if distance <= 2.0 && floor_translation.x > player_translation.x {
               let half_size = FLOOR_CUBE_SIZE / 2.0;
               commands.entity(entity)
                   .insert((RigidBody::Fixed,Collider::cuboid(half_size, half_size * floor.height, half_size)));
            }

            if distance < 0.5 && floor_translation.x < (player_translation.x - (FLOOR_CUBE_SIZE / 2.0)) {
                floor.height += FLOOR_CUBE_SIZE * 2.0;// * time.delta_seconds();

                for child_entity in children {
                    if let Ok(mut child_transform) = transforms.get_mut(*child_entity) {
                        child_transform.scale.y = floor.height;
                    }
                }
            }
        }
    }
}

pub fn shift_floors(
    mut commands: Commands,
    mut floors: Query<(Entity, &Floor, &Transform)>,
    cameras: Query<&Transform, With<Camera3d>>,
    mut floor_manager: ResMut<FloorManager>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut food_spawn_event_writer: EventWriter<food::SpawnFoodEvent>,
) {
    let mut rows = HashMap::<usize, Vec::<Floor>>::new();
    for (entity, floor, transform) in &mut floors {
        for camera in &cameras {
            if transform.translation.x < camera.translation.x {
                let floor = floor.clone();
                rows.entry(floor.row_id).and_modify(|r| r.push(floor)).or_insert(vec!(floor));
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    if !rows.is_empty() {
        let number_of_rows_to_add = rows.len() as i32;
        // does this need to be sorted?
        for (_, mut floors) in rows.drain() {
            floors.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());
            let row = FloorRow {
                blocks: floors.into()
            };

            floor_manager.floor_rows.push_back(row);
        }

        spawn_floors(&mut commands, &mut meshes, &mut materials, number_of_rows_to_add, &mut floor_manager, &mut food_spawn_event_writer);
    }
}

fn spawn_floors(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    number_of_rows_to_spawn: i32,
    floor_manager: &mut FloorManager,
    food_spawn_event_writer: &mut EventWriter<food::SpawnFoodEvent>,
) {
    for x in floor_manager.track_distance..(floor_manager.track_distance + number_of_rows_to_spawn) {
        if let Some(mut floor_row) = floor_manager.floor_rows.pop_front() {
            // reset the lowest when we run into the first row
            if let Some(front) = floor_row.blocks.front() {
                if front.row_id == 0 {
                    food_spawn_event_writer.send(food::SpawnFoodEvent);
                    floor_manager.lowest = f32::MAX;
                    floor_manager.highest = f32::MIN;
                }
            }

            while let Some(mut block) = floor_row.blocks.pop_front() {
                block.height *= 1.0 + DISTANCE_INCREASE;
                floor_manager.lowest = floor_manager.lowest.min(block.height);
                floor_manager.highest = floor_manager.highest.max(block.height);

//              println!("{} {}", block.height, floor_manager.lowest);
//              if block.height < floor_manager.lowest {
//                  // skipping because it's too low
//                  continue;
//              }
                
                let color_x = random_number();//* block.height;
                block.color.set_g(block.color.g() - (color_x * 0.1));
                block.color.set_r(block.color.r() - (color_x * 0.1));
                block.color.set_b(block.color.b() - (color_x * 0.1));
                commands.spawn((
                TransformBundle::from(Transform::from_xyz(x as f32 * FLOOR_CUBE_SIZE, 0.0, block.z as f32 * FLOOR_CUBE_SIZE)),
                block,
                ComputedVisibility::default(),
                Visibility::Visible,
                CleanupMarker,
    //          RigidBody::Fixed,
    //          Collider::cuboid(0.1, 0.1, 0.1)
                )).with_children(|parent| {
                    parent.spawn((
                        PbrBundle {
                            mesh: floor_manager.cube_mesh.clone_weak(),
                            material: materials.add(StandardMaterial {
                                base_color: block.color.into(),
                                unlit: false,
                                ..default()
                            }),
                            transform: Transform::from_scale(Vec3::new(1.0, block.height , 1.0)),
                            ..default()
                        }
                    ,));
                });
            }
        }
    }

    floor_manager.track_distance += number_of_rows_to_spawn;
}
