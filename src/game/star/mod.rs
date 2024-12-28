use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub const NUMBER_OF_STAR: usize = 10;
pub const STAR_SIZE: f32 = 30.0; // Star's size

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            // .add_startup_system(spawn_stars)
            // On Enter State
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // On Exit System
            .add_system(despawn_star.in_schedule(OnExit(AppState::Game)));
    }
}
