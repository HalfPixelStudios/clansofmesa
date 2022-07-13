use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{assetloader::*, camera::Cursor, game::*, map::*};

#[derive(Component)]
pub struct PlaceUI;

pub struct SelectUIPlugin;

impl Plugin for SelectUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_select_ui).add_system_set(
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
