use super::assetloader::*;
use super::game::*;
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

pub struct StructurePlugin;
impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnStructureEvent>()
            .add_system(spawn_structure)
            .add_startup_system(spawn_select_ui);
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
    mut cmd: Commands,
    inputs: Res<Vec<(NetInput, InputStatus)>>,
    mut spawn_event: EventWriter<SpawnStructureEvent>,
    mut select_query: Query<(&mut Transform, &mut TextureAtlasSprite), With<SelectUI>>,
    mut intgrid_query: Query<(&mut IntGridCell, &GridCoords), Without<SelectUI>>,
    cursor: Res<Cursor>,
    player: Res<LocalPlayerHandle>,
    game_data: Res<GameData>,
) {
    let mut can_build = true;
    let (mut transform, mut sprite) = select_query.single_mut();
    let (grid_x, grid_y) = snap_to_grid(cursor.0);

    let (ldtk_x, ldtk_y) = to_grid_coords(grid_x, grid_y);
    for (mut cell, coords) in intgrid_query.iter_mut() {
        sprite.color = Color::WHITE;
        if coords.x == ldtk_x && coords.y == ldtk_y {
            if cell.value == 1 {
                sprite.color = Color::BLACK;
                can_build = false;
            }
            break;
        }
    }

    transform.translation.x = grid_x as f32 - 8.;
    transform.translation.y = grid_y as f32 - 8.;
    transform.translation.z = 100.;

    let (input, _) = inputs[0];
    if (input.pressed & PLACE != 0) && can_build {
        spawn_event.send(SpawnStructureEvent {
            spawn_pos: Vec3::new(input.grid_x as f32, input.grid_y as f32, 100.),
            index: 0,
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
        info!("inserted cell");
    }
}
