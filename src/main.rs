pub mod events;
mod systems;

pub mod asteroid;
mod player;
pub mod score;
pub mod star;

use asteroid::AsteroidPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use events::*;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_plugin(AsteroidPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}
