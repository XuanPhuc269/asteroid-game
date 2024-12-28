use bevy::prelude::*;

pub const ASTEROID_SPAWN_TIME: f32 = 5.0;

#[derive(Resource)]
pub struct AsteroidSpawnTimer {
    pub timer: Timer,
}

impl Default for AsteroidSpawnTimer {
    fn default() -> Self {
        AsteroidSpawnTimer {
            timer: Timer::from_seconds(ASTEROID_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}