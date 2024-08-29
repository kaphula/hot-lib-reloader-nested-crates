use std::thread;
use std::time::Duration;
use components::Player;
use bevy_ecs::prelude::*;
use bevy_ecs::system::RunSystemOnce;


/// Hot-reloading is disabled, just import the systems the regular way:
#[cfg(not(feature = "reload"))]
use systems::*;

// Hot-reloading is enabled, import the hot-reload module below:
#[cfg(feature = "reload")]
use systems_hot::*;

// Setup hot-reloading from `systems` crate. Note that since we are running
// the hot-reloading in this crate (`main`) which is not at the project workspace root,
// we need to specify manually the `lib_dir` where the compiled dynamic library can be found.
// If not specified, the loader tries to load the dynamic library from `${project_root}/main/target/debug`
// which does not exist.
#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(
    dylib = "systems",
    lib_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/debug")

)]
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
        world.run_system_once(bevy_print_message_system);
        thread::sleep(Duration::from_millis(400));
    }
}
