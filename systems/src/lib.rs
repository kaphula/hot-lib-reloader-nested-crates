mod utilities;

use bevy::{prelude::*};
use components::*;
use rand::{thread_rng, Rng};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // player

    commands.spawn(Player {
        velocity: Vec3::ZERO,
        rotation_speed: f32::to_radians(180.0),
        shooting_timer: None,
    });
}

#[no_mangle]
pub fn bullet_hit_system(
    mut commands: Commands,
    bullet_query: Query<&Player>,
) {
    for player in bullet_query.iter() {
        println!("rotta {}", player.velocity)
    }
}

