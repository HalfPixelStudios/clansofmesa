pub mod ai;
pub mod prefab;

use bevy::prelude::*;
use bevy_bobs::{
    component::health::Health,
    prefab::{PrefabId, PrefabLib},
};
use bevy_ggrs::{Rollback, RollbackIdProvider};

use self::prefab::TowerPrefab;
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
        health: 100
    )
}
"#;

#[derive(Component, Clone)]
pub struct Tower;

pub struct SpawnStructureEvent {
    pub id: PrefabId,
    pub spawn_pos: Vec2,
}

pub struct StructurePlugin;
impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnStructureEvent>()
            .add_system(spawn_structure)
            .insert_resource(PrefabLib::<TowerPrefab>::new(RON_STRING));
    }
}

pub fn spawn_structure(
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
            .insert(Tower)
            .insert(Health::new(prefab.health))
            .insert(Rollback::new(rip.next_id()));
        }
    }
}
