use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ASTEROID_SPEED: f32 = 200.0;
pub const ASTEROID_SIZE: f32 = 64.0; // Asteroid's size

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsteroidSpawnTimer>() // Resources
            // Startup Systems
            // .add_startup_system(spawn_meteor)
            // Enter State Systems
            .add_system(spawn_meteor.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    asteroid_movement,
                    update_asteroid_direction,
                    confine_asteroid_movement,
                    tick_asteroid_spawn_timer,
                    spawn_asteroid_over_time,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State Systems
            .add_system(despawn_meteor.in_schedule(OnExit(AppState::Game)));
    }
}
