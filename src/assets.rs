use crate::asset_loading;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameAssets::default());
    }
}

#[derive(Default, Resource)]
pub struct GameAssets {
    pub font: Handle<Font>,
    pub donut: Handle<Gltf>,
    pub TJ: Handle<Gltf>,

    pub blip: Handle<AudioSource>,
    pub game_over: Handle<AudioSource>,
    pub jump: Handle<AudioSource>,
    pub collect: Handle<AudioSource>,
    pub collect_sfx: Handle<AudioSource>,
    pub level_bgm: Handle<AudioSource>,
    pub title_screen_bgm: Handle<AudioSource>,

    pub bevy_icon: asset_loading::GameTexture,
    pub title_screen_logo: asset_loading::GameTexture,
}
