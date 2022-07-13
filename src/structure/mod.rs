pub mod ai;
pub mod prefab;

use bevy::prelude::*;
use bevy_bobs::prefab::{PrefabId, PrefabLib};
use bevy_ecs_ldtk::prelude::*;
use bevy_ggrs::{Rollback, RollbackIdProvider};
use ggrs::InputStatus;

use self::prefab::TowerPrefab;
use crate::{
    assetloader::*,
    camera::*,
    game::*,
    input::*,
    layers::{LayerName, Layers},
    map::*,
};

const RON_STRING: &str = r#"
{
    "archer_tower": (
        cost: 100,
        sprite_index: 0,
        sprite_color: ColorRGB ( r: 1.0, g: 1.0, b: 1.0 ),
    )
}
"#;

#[derive(Component, Clone)]
pub struct Tower;

pub struct SpawnStructureEvent {
    pub id: PrefabId,
    pub spawn_pos: Vec2,
}

#[derive(Component)]
pub struct PlaceUI;

pub struct StructurePlugin;
impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnStructureEvent>()
            .add_system(spawn_structure)
            .insert_resource(PrefabLib::<TowerPrefab>::new(RON_STRING))
            .add_startup_system(spawn_select_ui)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_action)
                    .with_system(place_ui_controller),
            );
    }
}

pub fn spawn_select_ui(mut cmd: Commands, assets: Res<AssetSheet>) {
    cmd.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: 27,
            ..default()
        },
        texture_atlas: assets.0.clone(),
        ..default()
    })
    .insert(PlaceUI);
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
            .insert(Rollback::new(rip.next_id()));
        }
    }
}

pub fn place_ui_controller(
    mut place_ui_query: Query<(&mut Transform, &mut TextureAtlasSprite), With<PlaceUI>>,
    mut intgrid_query: Query<(&mut IntGridCell, &GridCoords), Without<PlaceUI>>,
    cursor: Res<Cursor>,
) {
    let (mut transform, mut sprite) = place_ui_query.single_mut();
    let (grid_x, grid_y) = snap_to_grid(cursor.0);

    let (ldtk_x, ldtk_y) = to_grid_coords(grid_x, grid_y);
    for (mut cell, coords) in intgrid_query.iter_mut() {
        sprite.color = Color::WHITE;
        if coords.x == ldtk_x && coords.y == ldtk_y {
            if cell.value == 1 {
                sprite.color = Color::BLACK;
            }
            break;
        }
    }

    transform.translation.x = grid_x as f32 - 8.;
    transform.translation.y = grid_y as f32 - 8.;
    transform.translation.z = 100.;
}

pub fn place_structure(
    mut cmd: Commands,
    inputs: Res<Vec<(NetInput, InputStatus)>>,
    mut spawn_event: EventWriter<SpawnStructureEvent>,
    mut intgrid_query: Query<(&mut IntGridCell, &GridCoords)>,
    game_data: Res<GameData>,
) {
    let (input, _) = inputs[game_data.defender];
    let mut can_build = true;

    let (ldtk_x, ldtk_y) = to_grid_coords(input.grid_x, input.grid_y);
    for (mut cell, coords) in intgrid_query.iter_mut() {
        if coords.x == ldtk_x && coords.y == ldtk_y {
            if cell.value == 1 {
                can_build = false;
            }
            break;
        }
    }

    if (input.pressed & PLACE != 0) && can_build {
        spawn_event.send(SpawnStructureEvent {
            id: "archer_tower".into(),
            spawn_pos: Vec2::new(input.grid_x as f32 - 8., input.grid_y as f32 - 8.),
        });
        for (mut cell, coords) in intgrid_query.iter_mut() {
            if coords.x == ldtk_x && coords.y == ldtk_y {
                cell.value = 1;
                return;
            }
        }
        cmd.spawn()
            .insert(IntGridCell { value: 1 })
            .insert(GridCoords {
                x: ldtk_x,
                y: ldtk_y,
            });
    }
}
