use bevy::prelude::*;

#[derive(Event)]
pub struct UnitEntered {
    pub entity: Entity,
    pub position: usize,
}

#[derive(Event)]
pub struct UnitExited(Entity);

#[derive(Event)]
pub struct TurnEnqueued {
    pub speed: f32,
    pub entity: Entity,
}
