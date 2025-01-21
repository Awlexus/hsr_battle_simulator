use bevy::prelude::*;
use crate::*;

pub struct DestructionTrailblazer;

impl Plugin for DestructionTrailblazer {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, add_trailblazer);
    }
}

fn add_trailblazer(mut commands: Commands) {
    commands.spawn(
        (
            Name("Destruction Trailblazer".to_string()),
            Character,
            Level(1),
            Health{max: 1203, current: 1203},
            Attack(620),
            Defense(460),
            Speed(100)
        )
    );
}
