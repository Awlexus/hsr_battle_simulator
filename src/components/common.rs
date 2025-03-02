use bevy::prelude::*;

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Level(pub u32);

#[derive(Component)]
pub struct Health {
    pub max: u32,
    pub current: u32,
}

#[derive(Component)]
pub struct Attack(pub f32);

#[derive(Component)]
pub struct Defense(pub f32);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Type(pub TypeEnum);

#[derive(Component)]
pub struct EffectHitRate(pub f32);

#[derive(Component)]
pub struct EffectResistance(pub f32);

enum TypeEnum {
    Physical,
    Fire,
    Ice,
    Lightning,
    Wind,
    Quantum,
    Imaginary,
}
