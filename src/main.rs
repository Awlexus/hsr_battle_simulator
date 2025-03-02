mod battle;
mod characters;
mod components;
mod enemies;

use bevy::prelude::*;
// use characters::destruction_trailblazer::DestructionTrailblazer;
use components::common::*;
use enemies::voidranger::Voidranger;

#[derive(Component)]
struct Enemy;

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .add_plugins(BattlePlugin)
        // .add_plugins(DestructionTrailblazer)
        // .add_plugins(Voidranger)
        .add_systems(Update, key_press_system)
        .run();
}

fn assign_positions(
    mut commands: Commands,
    mut character_query: Query<Entity, (With<Character>, Without<Position>)>,
    mut enemy_query: Query<Entity, (With<Enemy>, Without<Position>)>,
) {
    let mut index = 0;
    for character in character_query.iter() {
        commands.entity(character).insert(Position(index));
        index += 1;
    }

    index = 0;
    for enemy in enemy_query.iter() {
        commands.entity(enemy).insert(Position(index));
        index += 1;
    }
}

fn key_press_system(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        println!("Q key: Basic attack!")
    }

    if keyboard_input.just_pressed(KeyCode::KeyE) {
        println!("E key: Skill!")
    }

    if keyboard_input.just_pressed(KeyCode::KeyE) {
        println!("A key: Move selection to the left!")
    }

    if keyboard_input.just_pressed(KeyCode::KeyE) {
        println!("D key: Move selection to the right!")
    }
}
