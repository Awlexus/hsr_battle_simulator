use crate::components::common::*;
use crate::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct DestructionTrailblazer;

pub struct DestructionTrailblazerPlugin;

impl Plugin for DestructionTrailblazerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                basic,
                skill,
                ultimate,
                talent,
                passive_one,
                passive_two,
                passive_three,
            ),
        );
    }
}

// fn add_trailblazer(mut commands: Commands) {
//     commands.spawn(
//         (
//             Name("Destruction Trailblazer".to_string()),
//             Character,
//             Level(1),
//             Health{max: 1203, current: 1203},
//             Attack(620),
//             Defense(460),
//             Speed(100),
//             Path(PathEnum::Destruction),
//             Type(TypeEnum::Physical),

//         )
//     );
// }
