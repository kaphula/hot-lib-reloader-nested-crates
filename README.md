Demonstration of hot-reloading with `bevy_ecs` crate using [hot-lib-reloader-rs](https://github.com/rksm/hot-lib-reloader-rs?tab=readme-ov-file#hot-reloadable-functions-cannot-be-generic).

Run the following commands and change [main/src/systems/src/lib.rs](main/src/systems/src/lib.rs) `println!` statement at runtime:

```shell
# run dynamic library watcher in background:
cargo watch -w "main/src/systems" -w "main/src/components" -x "build -p systems" &

# run the main program with hot-reloading feature enabled:
cargo run --features reload
```
