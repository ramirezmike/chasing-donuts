use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::{
    direction,
    player,
    game_camera,
    floor,
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
                    player::handle_input, 
                    player::move_player,
                    floor::update_floors,
                    game_camera::follow_player,
                    floor::shift_floors,
                    apply_system_buffers
                ).chain()
                .in_set(OnUpdate(AppState::InGame))
            );
    }
}

fn reset_level(
) {
}
