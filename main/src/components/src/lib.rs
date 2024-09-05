use bevy_ecs::prelude::{Component, Resource};

#[derive(Component)]
pub struct OtherShip;

#[derive(Component)]
pub struct Bullet;

/// player component
#[derive(Component)]
pub struct Player {
    // pub movement_speed: f32,
    pub rotation_speed: f32,
}



/// player component
#[derive(Resource)]
pub struct BevyResource {
    // pub movement_speed: f32,
    pub x: f32,
}
