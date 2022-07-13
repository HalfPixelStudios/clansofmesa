use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use ggrs::InputStatus;

use crate::{enemy::*, game::*, input::*, map::*, structure::*};

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

    if (input.just_pressed & PLACE != 0) && can_build {
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
pub fn place_enemy(
    inputs: Res<Vec<(NetInput, InputStatus)>>,
    mut spawn_event: EventWriter<SpawnEnemyEvent>,
    mut intgrid_query: Query<(&mut IntGridCell, &GridCoords)>,
    game_data: Res<GameData>,
) {
    let (input, _) = inputs[game_data.attacker];
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

    if (input.just_pressed & PLACE != 0) && can_build {
        spawn_event.send(SpawnEnemyEvent {
            id: "nithin".into(),
            spawn_pos: Vec2::new(input.grid_x as f32 - 8., input.grid_y as f32 - 8.),
        });
    }
}
