pub mod ai;

use bevy::prelude::*;
use bevy_bobs::{component::health::Health, prefab::*};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EnemyPrefab {
    pub display_name: String,
    pub health: u32,
    pub reward: u32,
}

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

pub fn spawn_enemy_system(
    mut cmd: Commands,
    mut events: EventReader<SpawnEnemyEvent>,
    prefab_lib: Res<PrefabLib<EnemyPrefab>>,
) {
    for SpawnEnemyEvent { id, spawn_pos } in events.iter() {
        if let Some(prefab) = prefab_lib.get(id) {
            let e = cmd.spawn().id();
            cmd.entity(e).insert_bundle(EnemyBundle {
                enemy: Enemy,
                health: Health::new(prefab.health),
                reward: Reward(prefab.reward),
            });
        }
    }
}
