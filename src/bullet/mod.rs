pub mod prefab;

use crate::assetloader::AssetSheet;
use bevy::prelude::*;
use bevy_bobs::{
    component::lifetime::{DistanceLifetime, DurationLifetime, Lifetime, PenetrationLifetime},
    prefab::{models::*, *},
};

use self::prefab::*;

pub struct SpawnBulletEvent {
    pub id: PrefabId,
    pub spawn_pos: Vec2,
}

pub struct DespawnBulletEvent {
    pub entity: Entity,
    pub prefab: BulletPrefab,
}

#[derive(Component)]
pub struct Bullet(pub PrefabId);

#[derive(Component, Deref)]
pub struct Speed(pub f32);

#[derive(Component, Deref)]
pub struct Damage(pub u32);

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub speed: Speed,
    pub damage: Damage,
    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}

pub fn spawn_bullet_system(
    mut cmd: Commands,
    mut events: EventReader<SpawnBulletEvent>,
    prefab_lib: Res<PrefabLib<BulletPrefab>>,
    asset_sheet: Res<AssetSheet>,
) {
    for SpawnBulletEvent { id, spawn_pos } in events.iter() {
        if let Some(prefab) = prefab_lib.get(id) {
            let e = cmd.spawn().id();
            cmd.entity(e).insert_bundle(BulletBundle {
                bullet: Bullet(id.into()),
                speed: Speed(prefab.speed),
                damage: Damage(prefab.damage),
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

            // lifetimes
            let Lifetimes {
                distance,
                duration,
                penetration,
            } = prefab.lifetimes;
            if let Some(distance) = distance {
                cmd.entity(e).insert(DistanceLifetime::new(distance));
            }
            if let Some(duration) = duration {
                cmd.entity(e).insert(DurationLifetime::new(duration));
            }
            if let Some(penetration) = penetration {
                cmd.entity(e).insert(PenetrationLifetime::new(penetration));
            }
        }
    }
}

pub fn despawn_bullet_system(
    mut cmd: Commands,
    query: Query<(Entity, &Bullet)>,
    mut writer: EventWriter<DespawnBulletEvent>,
    prefab_lib: Res<PrefabLib<BulletPrefab>>,
) {
    for (entity, Bullet(id)) in query.iter() {
        // query all the lifetimes component has
        let mut is_expired = false;
        if let Ok(lifetime) = query.get_component::<DurationLifetime>(entity) {
            is_expired = if lifetime.is_expired() {
                true
            } else {
                is_expired
            };
        }
        if let Ok(lifetime) = query.get_component::<DistanceLifetime>(entity) {
            is_expired = if lifetime.is_expired() {
                true
            } else {
                is_expired
            };
        }
        if let Ok(lifetime) = query.get_component::<PenetrationLifetime>(entity) {
            is_expired = if lifetime.is_expired() {
                true
            } else {
                is_expired
            };
        }

        if is_expired {
            if let Some(prefab) = prefab_lib.get(id) {
                writer.send(DespawnBulletEvent {
                    prefab: prefab.clone(),
                    entity,
                });
                cmd.entity(entity).despawn();
            }
        }
    }
}
