pub mod ai;

use bevy::prelude::*;
use bevy_bobs::component::health::Health;

pub struct SpawnEnemyEvent {
    pub id: String,
    pub spawn_pos: Vec2,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Reward(pub u32);

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub health: Health,
    pub reward: Reward,
}

fn spawn_enemy_system(mut cmd: Commands, mut events: EventReader<SpawnEnemyEvent>) {}
