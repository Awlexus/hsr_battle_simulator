mod characters;
mod enemies;

use bevy::prelude::*;
use characters::destruction_trailblazer::DestructionTrailblazer;
use enemies::voidranger::Voidranger;

#[derive(Component)]
struct Character;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Level(u32);

#[derive(Component)]
struct Health{max: u32, current: u32}

#[derive(Component)]
struct Attack(u32);

#[derive(Component)]
struct Defense(u32);

#[derive(Component)]
struct Speed(u32);

#[derive(Component)]
struct Position(u32);

fn main() {
   App::new()
       .add_plugins(DefaultPlugins)
       .add_plugins(DestructionTrailblazer)
       .add_plugins(Voidranger)
       .add_systems(Update, key_press_system)
       .run();
}

fn key_press_system(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        println!("Q key: Basic attack!")
    }
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        println!("E key: Skill!")
    }
}
