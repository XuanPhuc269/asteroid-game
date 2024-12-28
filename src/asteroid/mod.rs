use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ASTEROID_SPEED: f32 = 200.0;
pub const ASTEROID_SIZE: f32 = 64.0; // Asteroid's size

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsteroidSpawnTimer>()
            .add_startup_system(spawn_meteor)
            .add_system(asteroid_movement)
            .add_system(update_asteroid_direction)
            .add_system(confine_asteroid_movement)
            .add_system(tick_asteroid_spawn_timer)
            .add_system(spawn_asteroid_over_time);
    }
}
