use bevy_ecs::{prelude::*};
use components::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Player {
        rotation_speed: f32::to_radians(180.0),
    });
}

#[no_mangle]
pub fn bevy_print_message_system(
    mut commands: Commands,
    bullet_query: Query<&Player>,
) {
    for player in bullet_query.iter() {
        println!("change this text during runtime {}", player.rotation_speed)
    }
}

