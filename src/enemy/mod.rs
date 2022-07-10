pub mod ai;

use autodefault::autodefault;
use bevy::prelude::*;
use bevy_bobs::{
    component::health::Health,
    prefab::{models::*, *},
};
use serde::Deserialize;

use crate::assetloader::*;

#[derive(Deserialize)]
pub struct EnemyPrefab {
    pub display_name: String,
    pub health: u32,
    pub reward: u32,
    pub sprite_index: usize,
    pub sprite_color: ColorRGB,
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

impl Default for EnemyBundle {
    fn default() -> Self {
        EnemyBundle {
            enemy: Enemy,
            health: Health::new(100),
            reward: Reward(0),
        }
    }
}

#[autodefault]
pub fn spawn_enemy_system(
    mut cmd: Commands,
    mut events: EventReader<SpawnEnemyEvent>,
    prefab_lib: Res<PrefabLib<EnemyPrefab>>,
    asset_sheet: Res<AssetSheet>,
) {
    for SpawnEnemyEvent { id, spawn_pos } in events.iter() {
        if let Some(prefab) = prefab_lib.get(id) {
            let e = cmd.spawn().id();
            cmd.entity(e)
                .insert_bundle(EnemyBundle {
                    enemy: Enemy,
                    health: Health::new(prefab.health),
                    reward: Reward(prefab.reward),
                })
                .insert_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: prefab.sprite_index,
                        color: prefab.sprite_color.into(),
                    },
                    texture_atlas: asset_sheet.0.clone(),
                    transform: Transform {
                        translation: spawn_pos.extend(0.),
                    },
                });
        }
    }
}
