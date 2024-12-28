use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::game::asteroid::components::*;
use crate::game::asteroid::resources::*;
use super::{NUMBER_OF_ENEMIES, ASTEROID_SIZE, ASTEROID_SPEED};

pub fn spawn_meteor(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // Define possible meteor textures
    let metor_textures = ["images/meteorGrey_big4.png", "images/meteorGrey_big3.png"];

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        // Randomly select a meteor texture
        let selected_texture = metor_textures[random::<usize>() % metor_textures.len()];

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load(selected_texture),
                ..default()
            },
            Asteroid {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn despawn_meteor(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Asteroid>>
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}
pub fn asteroid_movement(mut enemy_query: Query<(&mut Transform, &Asteroid)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ASTEROID_SPEED * time.delta_seconds();
    }
}

pub fn update_asteroid_direction(
    mut enemy_query: Query<(&Transform, &mut Asteroid)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_asteroid_size = ASTEROID_SIZE / 2.0;
    let x_min = 0.0 + half_asteroid_size;
    let x_max = window.width() - half_asteroid_size;
    let y_min = 0.0 + half_asteroid_size;
    let y_max = window.height() - half_asteroid_size;

    for (transform, mut asteroid) in enemy_query.iter_mut() {
        let mut direction_changed = false;

        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            asteroid.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            asteroid.direction.y *= -1.0;
            direction_changed = true;
        }

        // Play SFX
        if direction_changed {
            // Play sound effect
            let sound_effect_1: Handle<AudioSource> = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2: Handle<AudioSource> = asset_server.load("audio/pluck_002.ogg");
            // Randomly play one of the two sound effects
            let sound_effect: Handle<AudioSource> = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };
            audio.play(sound_effect);
        }
    }
}

pub fn confine_asteroid_movement(
    mut enemy_query: Query<&mut Transform, With<Asteroid>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_asteroid_size = ASTEROID_SIZE / 2.0;
    let x_min = 0.0 + half_asteroid_size;
    let x_max = window.width() - half_asteroid_size;
    let y_min = 0.0 + half_asteroid_size;
    let y_max = window.height() - half_asteroid_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        // Bound the asteroid x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound the asteroid y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn tick_asteroid_spawn_timer(
    mut asteroid_spawn_timer: ResMut<AsteroidSpawnTimer>,
    time: Res<Time>,
) {
    asteroid_spawn_timer.timer.tick(time.delta());
}



pub fn spawn_asteroid_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    asteroid_spawn_timer: ResMut<AsteroidSpawnTimer>,
) {
    if asteroid_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        // Define possible meteor textures
        let metor_textures = ["images/meteorGrey_big4.png", "images/meteorGrey_big3.png"];

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        // Randomly select a meteor texture
        let selected_texture = metor_textures[random::<usize>() % metor_textures.len()];

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load(selected_texture),
                ..default()
            },
            Asteroid {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}