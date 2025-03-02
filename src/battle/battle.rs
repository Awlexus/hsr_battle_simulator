use crate::{components::character::Character, Enemy, Speed};

use super::{
    battlefield::{Battlefield, Turn},
    events::{TurnEnqueued, UnitEntered, UnitExited},
};
use bevy::prelude::*;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Battlefield::default())
            .add_event::<UnitEntered>()
            .add_event::<UnitExited>()
            .add_event::<TurnEnqueued>()
            .add_systems(Update, (add_unit, enqueue_turn));
    }
}

fn add_unit(
    mut commands: Commands,
    mut entered: EventReader<UnitEntered>,
    mut battlefield: ResMut<Battlefield>,
    units: Query<(&Speed, Option<&Enemy>, Option<&Character>), Or<(With<Enemy>, With<Character>)>>,
) {
    for &UnitEntered { entity, position } in entered.read() {
        // Check if the entity is even there
        let Ok((&Speed(speed), enemy, character)) = units.get(entity) else {
            continue;
        };

        // Add entity to correct field
        if let Some(_) = enemy {
            battlefield.enemy_units.insert(position, entity);
        };
        if let Some(_) = character {
            battlefield.character_units.insert(position, entity);
        };

        // Add to action queue
        commands.send_event(TurnEnqueued { entity, speed });
    }
}

fn enqueue_turn(mut battlefield: ResMut<Battlefield>, mut enqueued: EventReader<TurnEnqueued>) {
    for &TurnEnqueued { speed, entity } in enqueued.read() {
        // Remove previous turn
        let previous = battlefield
            .action_values
            .iter()
            .position(|turn| turn.entity == entity)
            .map(|index| battlefield.action_values.remove(index));

        let action_value: f32 = match previous {
            Some(turn) => (turn.speed / speed) * turn.action_value,
            None => 10000.0 / speed,
        };

        // Check where new turn should be added
        let index = match battlefield
            .action_values
            .iter()
            .position(|turn| turn.action_value < action_value)
        {
            Some(index) => index,
            None => 0,
        };

        // Add the new turn
        battlefield.action_values.insert(
            index,
            Turn {
                action_value,
                speed,
                entity,
            },
        );
    }
}
