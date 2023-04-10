use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::gltf::Gltf;
use crate::{
    direction,
    player,
    game_camera,
    floor,
    asset_loading,
    assets,BACKGROUND_COLOR,
PLAYER_COLOR,
    food,
    AppState,
    ZeroSignum,
    cleanup,
    CleanupMarker,
};

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
//            .add_system(cleanup::<CleanupMarker>.in_schedule(OnExit(AppState::InGame)))
            .add_systems((
                    reset_level,
                )
                .in_set(OnUpdate(AppState::Reset))
            )
            .add_systems((
                    cleanup::<CleanupMarker>,
                )
                .in_schedule(OnEnter(AppState::Reset))
            )
            .add_system(setup.in_schedule(OnEnter(AppState::InGame)))
            .add_systems((
                    food::spawn_food,
                    food::update_food,
                )
                .in_set(OnUpdate(AppState::InGame))
            )
            .add_systems((
                    player::handle_input, 
                    player::move_player,
                    player::spin_mesh,
                    floor::update_floors,
                    game_camera::follow_player,
                    floor::shift_floors,
                    apply_system_buffers
                ).chain()
                .in_set(OnUpdate(AppState::InGame))
            );
    }
}

pub fn load(
    assets_handler: &mut asset_loading::AssetsHandler,
    game_assets: &mut ResMut<assets::GameAssets>,
) {
    assets_handler.add_font(&mut game_assets.font, "fonts/monogram.ttf");
    assets_handler.add_audio(&mut game_assets.collect, "audio/collect.wav");
    assets_handler.add_audio(&mut game_assets.jump, "audio/jump.wav");
    assets_handler.add_audio(&mut game_assets.game_over, "audio/game_over.wav");
    assets_handler.add_audio(&mut game_assets.game_over, "audio/game_over.wav");
    assets_handler.add_glb(&mut game_assets.TJ, "models/tj.glb");
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut floor_manager: ResMut<floor::FloorManager>,
    mut clear_color: ResMut<ClearColor>,
    mut food_spawn_event_writer: EventWriter<food::SpawnFoodEvent>,
    asset_server: Res<AssetServer>,
    game_assets: Res<assets::GameAssets>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    clear_color.0 = Color::hex(BACKGROUND_COLOR).unwrap();
    floor::setup_floor(&mut commands, &mut meshes, &mut materials, &mut floor_manager, &mut food_spawn_event_writer);

    if let Some(gltf) = assets_gltf.get(&game_assets.TJ) {
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
            ComputedVisibility::default(),
            Visibility::Visible,
            TransformBundle {
                local: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            },
            player::PlayerBundle::new(),
        )).with_children(|parent| {
            parent.spawn((SceneBundle { scene: gltf.scenes[0].clone(), ..default() }, player::InnerMesh));
        });
    }
        
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


fn reset_level(
    mut game_assets: ResMut<assets::GameAssets>,
    mut assets_handler: asset_loading::AssetsHandler,
) {
    println!("Resetting");
    assets_handler.load(AppState::InGame, &mut game_assets);
}
