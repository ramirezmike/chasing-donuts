#![windows_subsystem = "windows"]
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::*;
use rand::{thread_rng, Rng};
use bevy::winit::WinitSettings;
use bevy_rapier3d::render::RapierDebugRenderPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_inspector_egui::{quick::WorldInspectorPlugin, bevy_egui};

mod asset_loading;
mod assets;
mod player;
mod audio;
mod floor;
mod food;
mod game_camera;
mod direction;
mod game_over;
mod ingame;
mod ingame_ui;
mod menus;
mod splash;
mod title_screen;
mod ui;

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
        .add_state::<AppState>()
//        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(bevy_egui::EguiSettings { scale_factor: 1.8, ..default() })
//      .add_plugin(LogDiagnosticsPlugin::default())
//      .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
//        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(floor::FloorPlugin)
        .add_plugin(ingame::InGamePlugin)
        .add_plugin(audio::GameAudioPlugin)
        .add_plugin(ingame_ui::InGameUIPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(game_over::GameOverPlugin)
        .add_plugin(asset_loading::AssetLoadingPlugin)
        .add_plugin(title_screen::TitlePlugin)
        .add_plugin(splash::SplashPlugin)
        .add_plugin(ui::text_size::TextSizePlugin)
        .add_plugin(assets::AssetsPlugin)
        .add_plugin(food::FoodPlugin)
        .add_startup_system(window_settings)
        .add_system(bootstrap.in_set(OnUpdate(AppState::Initial)))
        .add_system(debug)
        .run();
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Initial,
    Loading,
    Splash,
    TitleScreen,
    GameOver,
    Reset,
    InGame,
}

fn bootstrap(
    mut assets_handler: asset_loading::AssetsHandler,
    mut game_assets: ResMut<assets::GameAssets>,
    mut clear_color: ResMut<ClearColor>,
    mut audio: audio::GameAudio,
) {
    audio.set_volume();
    clear_color.0 = Color::hex("aaaaaa").unwrap();

    //assets_handler.load(AppState::Splash, &mut game_assets);
    assets_handler.load(AppState::InGame, &mut game_assets);
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

pub fn random_in_f32_range(low: f32, high: f32) -> f32 {
    rand::thread_rng().gen_range(low..high)
}


#[derive(Component, Default)]
struct CleanupMarker;


use bevy::app::AppExit;
fn debug(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>, 
    mut exit: ResMut<Events<AppExit>>,
    players: Query<Entity, With<player::Player>>,
    light: Query<&Transform, With<DirectionalLight>>,
    mut food_event_writer: EventWriter<food::SpawnFoodEvent>,
 ) {
//  for l in &light {
//      println!("{:?}", l.rotation.to_axis_angle());
//  }
    if keys.just_pressed(KeyCode::Q) {
        exit.send(AppExit);
    }

    if keys.just_pressed(KeyCode::R) {
        food_event_writer.send(food::SpawnFoodEvent);
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
pub static FOOD_COLOR: &str = "ffb3cb";
