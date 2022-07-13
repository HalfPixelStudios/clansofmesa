pub mod ai;
pub mod prefab;

use bevy::prelude::*;
use bevy_bobs::{
    component::health::Health,
    prefab::{models::*, *},
};
use bevy_ggrs::{Rollback, RollbackIdProvider};

use self::{
    ai::{AIPlugin, DumbAI},
    prefab::*,
};
use crate::assetloader::*;

// temp define ron in string
const RON_STRING: &str = r#"
{
    "testing_enemy": (
        health: 100,
        reward: 20,
        ai: Dumb ( speed: 1. ),
        sprite_index: 1,
        sprite_color: ColorRGB ( r: 1.0, g: 1.0, b: 1.0 ),
    )
}
"#;

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

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AIPlugin)
            .insert_resource(PrefabLib::<EnemyPrefab>::new(RON_STRING))
            .add_event::<SpawnEnemyEvent>()
            .add_event::<DespawnEnemyEvent>()
            .add_startup_system(setup)
            .add_system(spawn_enemy_system)
            .add_system(despawn_enemy_system);
    }
}

fn setup(mut writer: EventWriter<SpawnEnemyEvent>) {
    writer.send(SpawnEnemyEvent {
        id: "testing_enemy".into(),
        spawn_pos: Vec2::ZERO,
    })
}

fn spawn_enemy_system(
    mut cmd: Commands,
    mut events: EventReader<SpawnEnemyEvent>,
    prefab_lib: Res<PrefabLib<EnemyPrefab>>,
    asset_sheet: Res<AssetSheet>,
    mut rip: ResMut<RollbackIdProvider>,
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

            match prefab.ai {
                AI::Dumb { speed } => {
                    cmd.entity(e).insert(DumbAI::new(speed));
                }
                _ => {}
            };

            cmd.entity(e).insert(Rollback::new(rip.next_id()));
        }
    }
}

fn despawn_enemy_system(
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
