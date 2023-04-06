#![windows_subsystem = "windows"]
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::*;
use rand::{thread_rng, Rng};
use bevy::winit::WinitSettings;
use bevy_rapier3d::render::RapierDebugRenderPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;
mod floor;
mod game_camera;
mod direction;

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
        .add_plugin(WorldInspectorPlugin::new())
//      .add_plugin(LogDiagnosticsPlugin::default())
//      .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(floor::FloorPlugin)
        .add_state::<AppState>()
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(window_settings)
        .add_startup_system(setup)
        .add_system(debug)
        .add_systems((
            game_camera::follow_player,
        ).in_set(OnUpdate(AppState::InGame)))
        .run();
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
    Initial,
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

fn random_number() -> f32 {
    let mut rng = thread_rng();
    let x: f32 = rng.gen();
    x * 2.0 - 1.0
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = meshes.add(Mesh::from(shape::Cube { size: 0.1 }));
    for x in 0..100 {
        for z in -10..10 {

//  for x in 0..1 {
//      for z in -1..1 {
            commands.spawn((
            TransformBundle::from(Transform::from_xyz(x as f32 * 0.1, 0.0, z as f32 * 0.1)),
            floor::Floor { height: 0.1 },
            ComputedVisibility::default(),
            Visibility::Visible,
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

    commands
        .spawn((
            RigidBody::KinematicPositionBased,
            Collider::cuboid(0.25, 0.25, 0.25),
            KinematicCharacterController {
                translation: Some(Vec3::new(0.0, 0.5, 0.0)),
                offset: CharacterLength::Absolute(0.01),
                ..default()
            },
           Velocity::default(),
//        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z | LockedAxes::ROTATION_LOCKED_Y,
        player::PlayerBundle::new(),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(0.8, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        }
    ));
        

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-1.8, 1.0, 0.0).looking_at(Vec3::new(8.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
}


use bevy::app::AppExit;
fn debug(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>, 
    mut exit: ResMut<Events<AppExit>>,
    players: Query<Entity, With<player::Player>>,
 ) {
    if keys.just_pressed(KeyCode::Q) {
        exit.send(AppExit);
    }

}


