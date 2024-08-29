use std::thread;
use std::time::Duration;
use components::Player;
use bevy_ecs::prelude::*;
use bevy_ecs::system::RunSystemOnce;
#[cfg(not(feature = "reload"))]
use systems::*;
#[cfg(feature = "reload")]
use systems_hot::*;

#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "systems")]
mod systems_hot {
    use bevy_ecs::prelude::*;
    pub use components::*;
    hot_functions_from_file!("main/src/systems/src/lib.rs");
}

fn main() {


    let mut world = World::new();

    world.spawn(Player {
        rotation_speed: 0.0,
    });

    loop {
        world.run_system_once(bullet_hit_system);
        thread::sleep(Duration::from_millis(400));
    }




}
