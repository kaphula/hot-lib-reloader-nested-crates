use bevy_ecs::{prelude::*};
use bevy_ecs::system::SystemState;
use components::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Player {
        rotation_speed: f32::to_radians(180.0),
    });
}

#[no_mangle]
pub fn bevy_print_message_system(
    bullet_query: Query<&Player>,
) {
    for player in bullet_query.iter() {
        println!(" this text during runtime {}", player.rotation_speed)
    }
}


#[no_mangle]
pub fn bevy_resource_test(
    world: &mut World
) {
    let mut ss = SystemState::<(
        ResMut<BevyResource>
    )>::new(world);

    let (mut res) = ss.get_mut(world);
    res.x = 520.0;
}
