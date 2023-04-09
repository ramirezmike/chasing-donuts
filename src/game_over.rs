use crate::{AppState, asset_loading, assets, floor, ui::text_size, CleanupMarker, menus, player, audio};

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(handle_game_over.in_set(OnUpdate(AppState::InGame)))
            .add_system(update_game_over.in_set(OnUpdate(AppState::GameOver)))
            .add_system(despawn_ui.in_schedule(OnEnter(AppState::GameOver)))
            .init_resource::<GameOverState>()
            .add_event::<GameOverEvent>();
    }
}

pub struct GameOverEvent;

#[derive(Resource)]
pub struct GameOverState {
    timer: f32,
    display_state: DisplayState 
}

enum DisplayState {
    GameOver,
    BaseScore,
    DonutsCollected,
    Distance,
    Height,
    Final,
    Continue,
    Wait
}

static COUNTDOWN: f32 = 0.2;

impl Default for GameOverState {
    fn default() -> Self {
        GameOverState {
            timer: COUNTDOWN,
            display_state: DisplayState::GameOver
        }
    }
}

#[derive(Component)]
struct GameOverContainer;

fn despawn_ui(
    nodes: Query<Entity, With<Node>>,
    mut commands: Commands,
) {
    for entity in &nodes {
        println!("Despawning");
        commands.get_or_spawn(entity).despawn_recursive();
    }
}

fn update_game_over(
    time: Res<Time>,
    mut commands: Commands,
    mut floor_manager: ResMut<floor::FloorManager>,
    mut game_over_state: ResMut<GameOverState>,
    mut game_over_containers: Query<Entity, With<GameOverContainer>>,
    text_scaler: text_size::TextScaler,
    players: Query<(&player::Player, &Transform)>,
    mut player_action_state: Query<&ActionState<player::PlayerAction>>,
    mut game_assets: ResMut<assets::GameAssets>,
    mut assets_handler: asset_loading::AssetsHandler,
    mut audio: audio::GameAudio,
) {
    game_over_state.timer -= time.delta_seconds();
    game_over_state.timer = game_over_state.timer.clamp(-3.0, 3.0);

    if game_over_state.timer > 0.0 {
        return;
    }

    println!("Done");
    game_over_state.timer = COUNTDOWN;
    let (player, player_transform) = players.single();
    let final_score = (floor_manager.score * player.donut_count.max(1)) as f32
        + (player_transform.translation.x * 10.0) 
        + (floor_manager.current_level_heights().1 * 100.0);
        

    match game_over_state.display_state {
        DisplayState::GameOver => {
            audio.play_sfx(&game_assets.game_over);
            game_over_state.display_state = DisplayState::BaseScore;
            commands
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Relative,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                    ..Default::default()
                })
                .insert(CleanupMarker)
                .insert(GameOverContainer)
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                                position_type: PositionType::Relative,
                                justify_content: JustifyContent::Center,
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    top: Val::Percent(5.),
                                    ..default()
                                },
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Row,
                                ..Default::default()
                            },
                            background_color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            add_title(
                                parent,
                                game_assets.font.clone(),
                                text_scaler.scale(menus::DEFAULT_FONT_SIZE * 1.6),
                                "GAME OVER",
                                vec!(CleanupMarker), // just an empty vec since can't do <impl Trait>
                            );
                        });
                });
        },
        DisplayState::BaseScore => {
            audio.play_sfx(&game_assets.game_over);
            game_over_state.display_state = DisplayState::DonutsCollected;
            for entity in &game_over_containers {
                let child = commands.spawn(
                            NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                                    position_type: PositionType::Relative,
                                    justify_content: JustifyContent::Center,
                                    margin: UiRect {
                                        left: Val::Auto,
                                        right: Val::Auto,
                                        ..default()
                                    },
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Row,
                                    ..Default::default()
                                },
                                background_color: Color::NONE.into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                add_title(
                                    parent,
                                    game_assets.font.clone(),
                                    text_scaler.scale(menus::DEFAULT_FONT_SIZE * 1.0),
                                    &format!("Base Score: {}", floor_manager.score),
                                    vec!(CleanupMarker), // just an empty vec since can't do <impl Trait>
                                );
                            })
                            .id();
                commands.entity(entity).add_child(child);
            }
        },
        DisplayState::DonutsCollected => {
            audio.play_sfx(&game_assets.game_over);
            game_over_state.display_state = DisplayState::Distance;
            for entity in &game_over_containers {
                let child = commands.spawn(
                            NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                                    position_type: PositionType::Relative,
                                    justify_content: JustifyContent::Center,
                                    margin: UiRect {
                                        left: Val::Auto,
                                        right: Val::Auto,
                                        ..default()
                                    },
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Row,
                                    ..Default::default()
                                },
                                background_color: Color::NONE.into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                add_title(
                                    parent,
                                    game_assets.font.clone(),
                                    text_scaler.scale(menus::DEFAULT_FONT_SIZE * 1.0),
                                    &format!("Donuts: {}", player.donut_count),
                                    vec!(CleanupMarker), // just an empty vec since can't do <impl Trait>
                                );
                            })
                            .id();
                commands.entity(entity).add_child(child);
            }
        },
        DisplayState::Distance => {
            audio.play_sfx(&game_assets.game_over);
            game_over_state.display_state = DisplayState::Height;
            for entity in &game_over_containers {
                let child = commands.spawn(
                            NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                                    position_type: PositionType::Relative,
                                    justify_content: JustifyContent::Center,
                                    margin: UiRect {
                                        left: Val::Auto,
                                        right: Val::Auto,
                                        ..default()
                                    },
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Row,
                                    ..Default::default()
                                },
                                background_color: Color::NONE.into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                add_title(
                                    parent,
                                    game_assets.font.clone(),
                                    text_scaler.scale(menus::DEFAULT_FONT_SIZE * 1.0),
                                    &format!("Distance: {:.2}", player_transform.translation.x),
                                    vec!(CleanupMarker), // just an empty vec since can't do <impl Trait>
                                );
                            })
                            .id();
                commands.entity(entity).add_child(child);
            }
        },
        DisplayState::Height => {
            audio.play_sfx(&game_assets.game_over);
            game_over_state.display_state = DisplayState::Final;
            for entity in &game_over_containers {
                let child = commands.spawn(
                            NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                                    position_type: PositionType::Relative,
                                    justify_content: JustifyContent::Center,
                                    margin: UiRect {
                                        left: Val::Auto,
                                        right: Val::Auto,
                                        ..default()
                                    },
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Row,
                                    ..Default::default()
                                },
                                background_color: Color::NONE.into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                add_title(
                                    parent,
                                    game_assets.font.clone(),
                                    text_scaler.scale(menus::DEFAULT_FONT_SIZE * 1.0),
                                    &format!("Max Height: {:.2}", floor_manager.current_level_heights().1),
                                    vec!(CleanupMarker), // just an empty vec since can't do <impl Trait>
                                );
                            })
                            .id();
                commands.entity(entity).add_child(child);
            }
        },
        DisplayState::Final => {
            audio.play_sfx(&game_assets.game_over);
            game_over_state.display_state = DisplayState::Continue;
            for entity in &game_over_containers {
                let child = commands.spawn(
                            NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                                    position_type: PositionType::Relative,
                                    justify_content: JustifyContent::Center,
                                    margin: UiRect {
                                        left: Val::Auto,
                                        right: Val::Auto,
                                        ..default()
                                    },
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Row,
                                    ..Default::default()
                                },
                                background_color: Color::NONE.into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                add_title(
                                    parent,
                                    game_assets.font.clone(),
                                    text_scaler.scale(menus::DEFAULT_FONT_SIZE * 1.0),
                                    &format!("Total Score: {}", final_score as i32),
                                    vec!(CleanupMarker), // just an empty vec since can't do <impl Trait>
                                );
                            })
                            .id();
                commands.entity(entity).add_child(child);
            }
        },
        DisplayState::Continue => {
            game_over_state.timer = -1.0;
            game_over_state.display_state = DisplayState::Wait;
            for entity in &game_over_containers {
                let child = commands.spawn(
                            NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                                    position_type: PositionType::Relative,
                                    justify_content: JustifyContent::Center,
                                    margin: UiRect {
                                        left: Val::Auto,
                                        right: Val::Auto,
                                        ..default()
                                    },
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Row,
                                    ..Default::default()
                                },
                                background_color: Color::NONE.into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                add_title(
                                    parent,
                                    game_assets.font.clone(),
                                    text_scaler.scale(menus::DEFAULT_FONT_SIZE * 1.0),
                                    "Again?",
                                    vec!(CleanupMarker), // just an empty vec since can't do <impl Trait>
                                );
                            })
                            .id();
                commands.entity(entity).add_child(child);
            }
        }
        DisplayState::Wait => {
            game_over_state.timer = -1.0;
            for action_state in &player_action_state {
                if action_state.just_pressed(player::PlayerAction::Action) {
                    *game_over_state = GameOverState::default();
                    assets_handler.load(AppState::Reset, &mut game_assets);
                }
            }
        }
    }
}

fn handle_game_over(
    mut event_reader: EventReader<GameOverEvent>,
    mut game_assets: ResMut<assets::GameAssets>,
    mut assets_handler: asset_loading::AssetsHandler,
) {
    if !event_reader.is_empty() {
        event_reader.clear();
        assets_handler.load(AppState::GameOver, &mut game_assets);
    }
}

pub fn add_title(
    builder: &mut ChildBuilder<'_, '_, '_>,
    font: Handle<Font>,
    font_size: f32,
    title: &str,
    mut components: Vec<impl Component>,
) {
    let mut text_bundle = builder.spawn(TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            margin: UiRect {
//              left: Val::Percent(2.0),
//              right: Val::Auto,
                ..Default::default()
            },
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        text: Text::from_section(
            title.to_string(),
            TextStyle {
                font,
                font_size,
                color: Color::WHITE,
            },
        ).with_alignment(TextAlignment::Center),
        ..Default::default()
    });

    components.drain(..).for_each(|c| {
        text_bundle.insert(c);
    });
}
