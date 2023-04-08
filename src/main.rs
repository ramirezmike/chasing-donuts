#![windows_subsystem = "windows"]
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::*;
use rand::{thread_rng, Rng};
use bevy::winit::WinitSettings;
use bevy_rapier3d::render::RapierDebugRenderPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_inspector_egui::{quick::WorldInspectorPlugin, bevy_egui};

mod player;
mod floor;
mod game_camera;
mod direction;
mod ingame;

fn main() {
  App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
          ..default()
        })
         .set(WindowPlugin {
          primary_window: Some(Window {
            fit_canvas_to_parent: true,
            ..default()
          }),
          ..default()
        }))
 //       .add_plugin(WorldInspectorPlugin::new())
 //       .insert_resource(bevy_egui::EguiSettings { scale_factor: 1.8, ..default() })
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
//        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(floor::FloorPlugin)
        .add_plugin(ingame::InGamePlugin)
        .add_state::<AppState>()
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(window_settings)
        .add_system(setup.in_schedule(OnEnter(AppState::InGame)))
        .add_system(debug)
        .run();
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
    Initial,
    Reset,
    #[default]
    InGame,
}

pub trait ZeroSignum {
    fn zero_signum(&self) -> Vec3;
}

impl ZeroSignum for Vec3 {
    fn zero_signum(&self) -> Vec3 {
        let convert = |n| {
            if n < 0.1 && n > -0.1 {
                0.0
            } else if n > 0.0 {
                1.0
            } else {
                -1.0
            }
        };

        Vec3::new(convert(self.x), convert(self.y), convert(self.z))
    }
}

fn window_settings(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    for mut window in windows.iter_mut() {
        window.title = String::from("Runner");
        //        window.set_mode(WindowMode::BorderlessFullscreen);
    }
}

pub fn random_number() -> f32 {
    let mut rng = thread_rng();
    let x: f32 = rng.gen();
    x * 2.0 - 1.0
}


#[derive(Component, Default)]
struct CleanupMarker;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut floor_manager: ResMut<floor::FloorManager>,
    mut clear_color: ResMut<ClearColor>,
) {
    clear_color.0 = Color::hex(BACKGROUND_COLOR).unwrap();
    floor::setup_floor(&mut commands, &mut meshes, &mut materials, &mut floor_manager);
    commands
        .spawn((
            RigidBody::KinematicPositionBased,
            Collider::cuboid(0.25, 0.25, 0.25),
            CleanupMarker,
            ColliderMassProperties::Density(2.0),
            KinematicCharacterController {
                translation: Some(Vec3::new(0.0, 0.5, 0.0)),
                offset: CharacterLength::Absolute(0.01),
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Absolute(1.0),
                    min_width: CharacterLength::Absolute(0.05),
                    include_dynamic_bodies: true,
                }),
                ..default()
            },
           Velocity::default(),
//        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z | LockedAxes::ROTATION_LOCKED_Y,
        player::PlayerBundle::new(),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::hex(PLAYER_COLOR).unwrap().into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        }
    ));
        
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.50,
    });

    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-1.8, 1.0, 0.0).looking_at(Vec3::new(8.0, 0.0, 0.0), Vec3::Y),
        ..default()
    },
    CleanupMarker,
    ComputedVisibility::default(),
    Visibility::Visible,
    ));

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::new(-0.8263363, -0.53950554, -0.16156079), 2.465743)),
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
//            illuminance: 10000.0,
            illuminance: 100000.0,
            shadows_enabled: false,
            ..Default::default()
        },
        ..Default::default()
    });
}


use bevy::app::AppExit;
fn debug(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>, 
    mut exit: ResMut<Events<AppExit>>,
    players: Query<Entity, With<player::Player>>,
    light: Query<&Transform, With<DirectionalLight>>,
 ) {
//  for l in &light {
//      println!("{:?}", l.rotation.to_axis_angle());
//  }
    if keys.just_pressed(KeyCode::Q) {
        exit.send(AppExit);
    }

    if keys.just_pressed(KeyCode::R) {
    }
}

pub fn cleanup<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in entities.iter() {
        commands.get_or_spawn(entity).despawn_recursive();
    }
}

pub static FLOOR_COLOR: &str = "d8bfd8";
pub static PLAYER_COLOR: &str = "96fbc7";
pub static BACKGROUND_COLOR: &str = "74569b";
