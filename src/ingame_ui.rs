use crate::{
    assets::GameAssets, menus, AppState, ui::text_size, ingame, floor,CleanupMarker,
};
use bevy::prelude::*;

pub struct InGameUIPlugin;
impl Plugin for InGameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::InGame)))
           .add_system(update_ui.in_set(OnUpdate(AppState::InGame)));
    }
}

fn update_ui(
    floor_manager: Res<floor::FloorManager>,
    game_assets: Res<GameAssets>,
    mut score_indicators: Query<&mut Text, With<ScoreIndicator>>,
) {
    for mut score in &mut score_indicators {
        score.sections[0].value = format!("{}", floor_manager.score);
    }
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut floor_manager: ResMut<floor::FloorManager>,
    text_scaler: text_size::TextScaler,
) {
    println!("Setting up UI");
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(CleanupMarker)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(50.0), Val::Percent(15.0)),
                        position_type: PositionType::Relative,
                        justify_content: JustifyContent::FlexStart,
                        margin: UiRect {
                            left: Val::Percent(2.0),
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
                        text_scaler.scale(menus::DEFAULT_FONT_SIZE * 0.6),
                        "Score: ",
                        Vec::<CleanupMarker>::new(), // just an empty vec since can't do <impl Trait>
                    );
                    add_title(
                        parent,
                        game_assets.font.clone(),
                        text_scaler.scale(menus::DEFAULT_FONT_SIZE * 0.6),
                        "0%",
                        vec!(ScoreIndicator), // just an empty vec since can't do <impl Trait>
                    );
                });
        });
}


#[derive(Component)]
struct ScoreIndicator;

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
