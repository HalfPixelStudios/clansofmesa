use super::assetloader::*;
use bevy_ecs_ldtk::prelude::*;

use super::camera::*;
use super::input::*;
use super::map::*;
use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackIdProvider};
use ggrs::InputStatus;
#[derive(Component, Clone)]
pub struct Tower;

pub struct SpawnStructureEvent {
    pub spawn_pos: Vec3,
    pub index: usize,
}
#[derive(Component)]
pub struct SelectUI;

pub struct BuildData {
    placeable: bool,
}

pub struct StructurePlugin;
impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnStructureEvent>()
            .add_system(spawn_structure)
            .add_startup_system(spawn_select_ui)
            .add_system(check_placeable)
            .insert_resource(BuildData { placeable: false });
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
    .insert(SelectUI);
}

pub fn check_placeable(
    mut select_query: Query<(&mut Transform, &mut TextureAtlasSprite), With<SelectUI>>,
    mut intgrid_query: Query<(&mut IntGridCell, &GridCoords), Without<SelectUI>>,
    cursor: Res<Cursor>,
    mut build_data: ResMut<BuildData>,
) {
    let (mut transform, mut sprite) = select_query.single_mut();
    let (grid_x, grid_y) = snap_to_grid(cursor.0);

    let (ldtk_x, ldtk_y) = to_grid_coords(grid_x, grid_y);
    build_data.placeable = true;
    for (cell, coords) in intgrid_query.iter() {
        sprite.color = Color::WHITE;
        if coords.x == ldtk_x && coords.y == ldtk_y {
            info!("{:?}", cell.value);
            if cell.value == 1 {
                build_data.placeable = false;
                sprite.color = Color::BLACK;
                break;
            }
        }
    }

    transform.translation.x = grid_x as f32 - 8.;
    transform.translation.y = grid_y as f32 - 8.;
    transform.translation.z = 100.;
}

pub fn spawn_structure(
    mut cmd: Commands,
    assets: Res<AssetSheet>,
    mut event: EventReader<SpawnStructureEvent>,
    mut rip: ResMut<RollbackIdProvider>,
) {
    for ev in event.iter() {
        cmd.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: ev.index,
                ..default()
            },
            transform: Transform {
                translation: ev.spawn_pos,
                ..default()
            },
            texture_atlas: assets.0.clone(),
            ..default()
        })
        .insert(Tower)
        .insert(Rollback::new(rip.next_id()));
    }
}

pub fn place_structure(
    inputs: Res<Vec<(NetInput, InputStatus)>>,
    mut spawn_event: EventWriter<SpawnStructureEvent>,
    build_data: Res<BuildData>,
) {
    let (input, _) = inputs[0];
    if (input.pressed & PLACE != 0) && build_data.placeable {
        spawn_event.send(SpawnStructureEvent {
            spawn_pos: Vec3::new(input.grid_x as f32, input.grid_y as f32, 0.),
            index: 0,
        });
    }
}
