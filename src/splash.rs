use bevy::prelude::*;
use crate::{AppState, ui::text_size, assets::GameAssets, menus, cleanup, asset_loading, };

pub struct SplashPlugin;
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::Splash)))
            .init_resource::<SplashTracker>()
            .add_system(tick.in_set(OnUpdate(AppState::Splash)))
            .add_system(cleanup::<CleanupMarker>.in_schedule(OnExit(AppState::Splash))
        );
    }
}

#[derive(Component)]
struct CleanupMarker;

#[derive(Default, Resource)]
struct SplashTracker {
    time: f32
}

pub fn load(
    assets_handler: &mut asset_loading::AssetsHandler,
    game_assets: &mut ResMut<GameAssets>,
) {
    assets_handler.add_material(&mut game_assets.bevy_icon, "textures/bevy.png", true);
    assets_handler.add_font(&mut game_assets.font, "fonts/monogram.ttf");
}

fn tick(
    time: Res<Time>,
    mut splash_tracker: ResMut<SplashTracker>,
    mut game_assets: ResMut<GameAssets>,
    mut assets_handler: asset_loading::AssetsHandler,
) {
    splash_tracker.time += time.delta_seconds();

    if splash_tracker.time > 3.0 {
        assets_handler.load(AppState::TitleScreen, &mut game_assets);
    }
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    text_scaler: text_size::TextScaler,
    mut splash_tracker: ResMut<SplashTracker>,
    mut clear_color: ResMut<ClearColor>,
) {
    splash_tracker.time = 0.0;
    clear_color.0 = Color::hex("000000").unwrap();

    commands
        .spawn(Camera3dBundle {
            ..Default::default()
        })
        .insert(CleanupMarker);

   commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Relative,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                flex_direction: FlexDirection::Column,
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CleanupMarker)
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Percent(60.0)),
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                image: game_assets.bevy_icon.image.clone().into(),
                ..Default::default()
            });

            parent.spawn(TextBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::Center,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::from_section(
                    "made with Bevy",
                    TextStyle {
                        font: game_assets.font.clone(),
                        font_size: text_scaler.scale(menus::DEFAULT_FONT_SIZE * 1.2),
                        color: Color::WHITE,
                    })
                    .with_alignment(TextAlignment::Center),
                    ..Default::default()
                });
            });
}


