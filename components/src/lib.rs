use bevy_ecs::prelude::Component;

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
