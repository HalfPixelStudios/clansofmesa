pub mod ai;
pub mod prefab;

use bevy::prelude::*;
use bevy_bobs::{
    component::health::Health,
    prefab::{models::*, *},
};

use self::prefab::*;
use crate::assetloader::*;

pub struct SpawnEnemyEvent {
    pub id: PrefabId,
    pub spawn_pos: Vec2,
}

pub struct DespawnEnemyEvent {
    pub entity: Entity,
    pub prefab: EnemyPrefab,
}

#[derive(Component)]
pub struct Enemy(pub PrefabId);

#[derive(Component)]
pub struct Reward(pub u32);

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub health: Health,
    pub reward: Reward,
    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}

pub fn spawn_enemy_system(
    mut cmd: Commands,
    mut events: EventReader<SpawnEnemyEvent>,
    prefab_lib: Res<PrefabLib<EnemyPrefab>>,
    asset_sheet: Res<AssetSheet>,
) {
    for SpawnEnemyEvent { id, spawn_pos } in events.iter() {
        if let Some(prefab) = prefab_lib.get(id) {
            let e = cmd.spawn().id();
            cmd.entity(e).insert_bundle(EnemyBundle {
                enemy: Enemy(id.into()),
                health: Health::new(prefab.health),
                reward: Reward(prefab.reward),
                sprite_sheet: SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: prefab.sprite_index,
                        color: prefab.sprite_color.into(),
                        ..default()
                    },
                    texture_atlas: asset_sheet.0.clone(),
                    transform: Transform {
                        translation: spawn_pos.extend(0.),
                        ..default()
                    },
                    ..default()
                },
            });
        }
    }
}

pub fn despawn_enemy_system(
    mut cmd: Commands,
    query: Query<(Entity, &Enemy, &Health)>,
    mut writer: EventWriter<DespawnEnemyEvent>,
    prefab_lib: Res<PrefabLib<EnemyPrefab>>,
) {
    for (entity, Enemy(id), health) in query.iter() {
        if health.is_zero() {
            if let Some(prefab) = prefab_lib.get(id) {
                writer.send(DespawnEnemyEvent {
                    prefab: prefab.clone(),
                    entity,
                });
                cmd.entity(entity).despawn();
            }
        }
    }
}
