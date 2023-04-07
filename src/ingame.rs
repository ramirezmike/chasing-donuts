use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::{
    direction,
    player,
    AppState,
    ZeroSignum,
    cleanup,
    CleanupMarker,
};

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(cleanup::<CleanupMarker>.in_schedule(OnExit(AppState::InGame)))
            .add_systems((
                    reset_level,
                )
                .in_set(OnUpdate(AppState::Reset))
            );
    }
}

fn reset_level(
) {
}
