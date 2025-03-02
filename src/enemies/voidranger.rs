use bevy::prelude::*;

pub struct Voidranger;

impl Plugin for Voidranger {
    fn build(&self, app: &mut App) {
        app;
        // app.add_systems(Startup, add_voidranger);
    }
}

// fn add_voidranger(mut commands: Commands) {
//     commands.spawn((
//         Name("Voidranger".to_string()),
//         Enemy,
//         Level(1),
//         Health {
//             max: 134,
//             current: 134,
//         },
//         Attack(12),
//         Defense(210),
//         Speed { field1: 120 },
//         Type(TypeEnum::Quantum),
//     ));
// }
