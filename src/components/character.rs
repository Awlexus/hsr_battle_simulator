use bevy::prelude::*;

use super::common;
use super::common::*;

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct CritRate(pub f32);

#[derive(Component)]
pub struct CritDamage(pub f32);

#[derive(Component)]
pub struct BreakEffect(pub f32);

#[derive(Component)]
pub struct Energy {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct OutgoingHealingBoost(pub f32);

#[derive(Component)]
pub struct EnergyRegenerationRate(pub f32);

#[derive(Component)]
pub struct Path(PathEnum);

#[derive(Component)]
pub struct BasicATK(u32);

#[derive(Component)]
pub struct Skill(u32);

#[derive(Component)]
pub struct Ultimate(u32);

#[derive(Component)]
pub struct Talent(u32);

#[derive(Component)]
pub struct MajorTrace(u32);

#[derive(Component)]
pub struct Eidolon(u32);

pub enum AbilityType {
    SingleTarget,
    Blast,
    AOE,
}

pub enum PathEnum {
    Destruction,
    TheHunt,
    Erudition,
    Harmony,
    Nihility,
    Preservation,
    Abundance,
    Remembrance,
}
