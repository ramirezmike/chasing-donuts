use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::{
    direction,
    player,
    AppState,
    ZeroSignum,
};

static FLOOR_CUBE_SIZE: f32 = 0.1;
static GROUND_SPEED: f32 = 1.0;

pub struct FloorPlugin;
impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems((
                    update_floors,
                )
                .in_set(OnUpdate(AppState::InGame))
            );
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
            if distance < 1.0 {

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
            } else {
                commands.entity(entity)
                        .remove::<RigidBody>()
                        .remove::<Collider>();

            }
        }
    }
}
