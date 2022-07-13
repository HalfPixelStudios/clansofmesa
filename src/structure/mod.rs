pub mod ai;
pub mod prefab;

use bevy::prelude::*;
use bevy_bobs::{
    attack_pattern::AttackPattern,
    component::health::Health,
    prefab::{PrefabId, PrefabLib},
};
use bevy_ggrs::{Rollback, RollbackIdProvider};

use self::{
    ai::AttackAI,
    prefab::{AttackPreference, TowerPrefab},
};
use crate::{
    assetloader::*,
    layers::{LayerName, Layers},
};

const RON_STRING: &str = r#"
{
    "archer_tower": (
        cost: 100,
        sprite_index: 0,
        sprite_color: ColorRGB ( r: 1.0, g: 1.0, b: 1.0 ),
        health: 100,
    )
}
"#;

#[derive(Component, Clone)]
pub struct Tower(PrefabId);

pub struct SpawnStructureEvent {
    pub id: PrefabId,
    pub spawn_pos: Vec2,
}

pub struct StructurePlugin;
impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnStructureEvent>()
            .add_startup_system(setup)
            .add_system(spawn_structure_system)
            .add_system(despawn_structure_system)
            .insert_resource(PrefabLib::<TowerPrefab>::new(RON_STRING));
    }
}

fn setup(mut writer: EventWriter<SpawnStructureEvent>) {
    writer.send(SpawnStructureEvent {
        id: "archer_tower".into(),
        spawn_pos: Vec2::ZERO,
    });
}

fn spawn_structure_system(
    mut cmd: Commands,
    prefab_lib: Res<PrefabLib<TowerPrefab>>,
    asset_sheet: Res<AssetSheet>,
    mut events: EventReader<SpawnStructureEvent>,
    mut rip: ResMut<RollbackIdProvider>,
    layers: Res<Layers>,
) {
    for SpawnStructureEvent { id, spawn_pos } in events.iter() {
        if let Some(prefab) = prefab_lib.get(id) {
            cmd.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: prefab.sprite_index,
                    color: prefab.sprite_color.into(),
                    ..default()
                },
                texture_atlas: asset_sheet.0.clone(),
                transform: Transform {
                    translation: spawn_pos.extend(layers.get(LayerName::Tower).z_height),
                    ..default()
                },
                ..default()
            })
            .insert(Tower(id.into()))
            .insert(Health::new(prefab.health))
            .insert(AttackAI {
                bullet_id: "archer_bullet".into(),
                preference: AttackPreference::Closest,
                attack_range: 400.,
                attack_pattern: AttackPattern::Straight,
            })
            .insert(Rollback::new(rip.next_id()));
        }
    }
}

fn despawn_structure_system(mut cmd: Commands, query: Query<(Entity, &Tower, &Health)>) {
    for (entity, Tower(id), health) in query.iter() {
        if health.is_zero() {
            cmd.entity(entity).despawn();
        }
    }
}
