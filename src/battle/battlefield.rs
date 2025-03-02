use bevy::prelude::*;

#[derive(Resource)]
pub struct Battlefield {
    pub action_values: Vec<Turn>,
    pub enemy_units: Vec<Entity>,
    pub character_units: Vec<Entity>,
}

pub struct Turn {
    pub action_value: f32,
    pub speed: f32,
    pub entity: Entity,
}

// impl Battlefield {
//     pub fn insert_unit_action(&mut self, speed: f32, entity: Entity) {
//         let action_value = 10000 / speed
//         self.action_values.insert
//     }
// }

impl Default for Battlefield {
    fn default() -> Self {
        Battlefield {
            action_values: Vec::new(),
            enemy_units: Vec::new(),
            character_units: Vec::new(),
        }
    }
}
