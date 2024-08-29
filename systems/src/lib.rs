mod utilities;

use bevy_ecs::{prelude::*};
use components::*;
use rand::{thread_rng, Rng};

pub fn setup(mut commands: Commands) {
    // player

    commands.spawn(Player {
        rotation_speed: f32::to_radians(180.0),
    });
}

#[no_mangle]
pub fn bullet_hit_system(
    mut commands: Commands,
    bullet_query: Query<&Player>,
) {
    for player in bullet_query.iter() {
        println!("jokeri {}", player.rotation_speed)
    }
}

