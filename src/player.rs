use crate::{
    direction,
    AppState,assets,
    ZeroSignum,
    floor,
    audio,
    game_over,
};
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::TAU;
use std::collections::HashMap;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::plugin::InputManagerSystem;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_event::<PlayerMoveEvent>();
    }
}

#[derive(Component, Reflect, Default)]
pub struct InnerMesh;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
    pub speed: f32,
    pub rotation_speed: f32,
    pub friction: f32,
    pub velocity: Vec3,
    pub random: f32,
    pub state: PlayerState,
    pub death_timer: Option<f32>,
    pub donut_count: usize,
    pub jump_cooldown: f32,
}

impl Player {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        Player {
            speed: 20.0,
            rotation_speed: 1.0,
            friction: 0.1,
            velocity: Vec3::ZERO,
            random: rng.gen_range(0.5..1.0),
            state: PlayerState::Normal,
            death_timer: None,
            donut_count: 0,
            jump_cooldown: 0.0,
        }
    }
}

pub enum Movement {
    Normal(direction::Direction),
    Jump,
}

#[derive(Reflect, Clone, PartialEq)]
pub enum PlayerState {
    Normal,
}

impl Default for PlayerState {
    fn default() -> PlayerState {
        PlayerState::Normal
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,

    Action,
}
impl PlayerAction {
    const DIRECTIONS: [Self; 4] = [
        PlayerAction::Up,
        PlayerAction::Down,
        PlayerAction::Left,
        PlayerAction::Right,
    ];

    fn direction(self) -> direction::Direction {
        match self {
            PlayerAction::Up => direction::Direction::UP,
            PlayerAction::Down => direction::Direction::DOWN,
            PlayerAction::Left => direction::Direction::LEFT,
            PlayerAction::Right => direction::Direction::RIGHT,
            _ => direction::Direction::NEUTRAL,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    #[bundle]
    input_manager: InputManagerBundle<PlayerAction>,
}

impl PlayerBundle {
    pub fn new() -> Self {
        PlayerBundle {
            player: Player::new(),
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::default_input_map(),
                action_state: ActionState::default(),
            },
        }
    }

    fn default_input_map() -> InputMap<PlayerAction> {
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        input_map.set_gamepad(Gamepad { id: 0 });

        // Movement
//      input_map.insert(KeyCode::Up, Up);
//      input_map.insert(KeyCode::W, Up);
//      input_map.insert(KeyCode::Z, Up);
//      input_map.insert(GamepadButtonType::DPadUp, Up);

//      input_map.insert(KeyCode::Down, Down);
//      input_map.insert(KeyCode::S, Down);
//      input_map.insert(GamepadButtonType::DPadDown, Down);

        input_map.insert(KeyCode::Left, Left);
        input_map.insert(KeyCode::A, Left);
        input_map.insert(KeyCode::Q, Left);
        input_map.insert(GamepadButtonType::DPadLeft, Left);

        input_map.insert(KeyCode::Right, Right);
        input_map.insert(KeyCode::D, Right);
        input_map.insert(GamepadButtonType::DPadRight, Right);

        // Actions
        input_map.insert(KeyCode::J, Action);
        input_map.insert(KeyCode::Space, Action);
        input_map.insert(KeyCode::Return, Action);
        input_map.insert(GamepadButtonType::South, Action);

        input_map
    }
}

pub struct PlayerMoveEvent {
    pub entity: Entity,
    pub movement: Movement,
}

pub fn handle_input(
    mut app_state: ResMut<State<AppState>>,
    mut players: Query<(Entity, &ActionState<PlayerAction>, &Transform, &mut Player, &mut Velocity)>,
    mut player_move_event_writer: EventWriter<PlayerMoveEvent>,
) {
    for (entity, action_state, transform, mut player, mut velocity) in &mut players {
        //println!("T: {:?}", transform.translation);
        let mut direction = direction::Direction::NEUTRAL;

        if action_state.just_pressed(PlayerAction::Action) {
            player_move_event_writer.send(PlayerMoveEvent {
                entity,
                movement: Movement::Jump,
            });
        }
        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.pressed(input_direction) {
                direction += input_direction.direction();
            }
        }

        if direction != direction::Direction::NEUTRAL {
            player_move_event_writer.send(PlayerMoveEvent {
                entity,
                movement: Movement::Normal(direction),
            });
        }

    }
}

pub fn move_player(
    time: Res<Time>,
    mut players: Query<(Entity, &mut KinematicCharacterController, &KinematicCharacterControllerOutput, &mut Transform, &mut Player, &mut Velocity), Without<Camera3d>>,
    mut player_move_event_reader: EventReader<PlayerMoveEvent>,
    mut game_over_event_writer: EventWriter<game_over::GameOverEvent>,
    floor_manager: Res<floor::FloorManager>,
    game_assets: Res<assets::GameAssets>,
    mut audio: audio::GameAudio,
) {
    let mut move_events = HashMap::new();
    for move_event in player_move_event_reader.iter() {
        move_events.entry(move_event.entity).or_insert(move_event);
    }

    for (entity, mut controller, controller_output, mut transform, mut player, p_velocity) in players.iter_mut() {

        if p_velocity.linvel.x < player.speed * 0.1 {
            let current_death_time = player.death_timer.unwrap_or(3.0) - time.delta_seconds();

            if current_death_time < 0.0 {
                game_over_event_writer.send(game_over::GameOverEvent);
            } else {
                player.death_timer = Some(current_death_time);
            }
        } else {
            player.death_timer = None;
        }

        transform.rotate_z(time.delta_seconds());

        let lowest = floor_manager.get_actual_lowest(); 
        if transform.translation.y < lowest - 3.0 {
            game_over_event_writer.send(game_over::GameOverEvent);
        }

        let speed: f32 = player.speed;
        let rotation_speed: f32 = player.rotation_speed;
        let friction: f32 = player.friction;
        let mut gravity: Vec3 = 3.0 * Vec3::new(0.0, -1.0, 0.0);

        player.velocity *= friction.powf(time.delta_seconds());
        player.velocity += (Vec3::X * speed) * time.delta_seconds();

        if controller_output.grounded {
            player.jump_cooldown = player.jump_cooldown.max(0.2);
        }
        player.jump_cooldown -= time.delta_seconds();
        player.jump_cooldown = player.jump_cooldown.clamp(-3.0, 3.0);

        if let Some(move_event) = move_events.get(&entity) {
            match move_event.movement {
                Movement::Normal(direction) => {
                    let mut acceleration = Vec3::from(direction).zero_signum();
                    if !controller_output.grounded {
                        acceleration.z *= 0.5;
                    }
                    player.velocity += (acceleration * speed) * time.delta_seconds();
                },
                Movement::Jump => {
                    if player.jump_cooldown > 0.0 {
                        println!("JUMP");
                        player.jump_cooldown = 0.0;
                        audio.play_sfx(&game_assets.jump);
                        player.velocity += Vec3::new(0.0, 300.0, 0.0) * time.delta_seconds();
                        gravity = Vec3::ZERO;
                    }
                },
            }
        }

        player.velocity = player.velocity.clamp_length_max(speed);

//      player.velocity.z *= if player.velocity.x > 0.0 { 1.0 } else { 0.0 };
//      player.velocity.y *= if player.velocity.x > 0.0 { 1.0 } else { 0.0 };
//      game_state.driving_speed = player.velocity.x * 0.1;

        let new_translation = (gravity + player.velocity) * time.delta_seconds();

        let angle = (-(new_translation.z - transform.translation.z))
            .atan2(new_translation.x - transform.translation.x);
        let rotation = Quat::from_axis_angle(Vec3::Y, angle);
//       velocity.angvel = rotation.to_scaled_axis();
        controller.translation = Some(new_translation);
//        velocity.linvel = player.velocity * time.delta_seconds();

//        transform.translation.x = 0.0; // hardcoding for now

        let new_rotation = transform
            .rotation
            .lerp(Quat::from_axis_angle(Vec3::Y, TAU * 0.75), time.delta_seconds() * rotation_speed);

        // don't rotate if we're not moving or if rotation isnt a number
        if !rotation.is_nan() && player.velocity.length() > 1.0 {
            transform.rotation = rotation;
        }
    }
}

pub fn spin_mesh( 
    player: Query<(&Velocity, &Player)>,
    time: Res<Time>,
    mut inner_mesh: Query<&mut Transform, With<InnerMesh>>,
) {
    for (v, player) in &player {
        if v.linvel.x > player.speed * 0.1 {
            for mut i in &mut inner_mesh {
                i.rotate_z(time.delta_seconds() * player.speed);
            }
        }
    }
}

