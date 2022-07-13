use super::camera::*;
use super::game::*;
use super::map::*;
use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use ggrs::PlayerHandle;

pub type Pack = i32;

pub const PLACE: Pack = 1 << 4;
pub const CAMERA: Pack = 1 << 2;
pub const ACTION: Pack = 1 << 3;
pub const INSPECT: Pack = 1 << 5;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct NetInput {
    pub pressed: Pack,
    pub just_pressed: Pack,

    pub grid_x: i32,
    pub grid_y: i32,
}

// TODO can define user mapped keys here
pub fn input_system(
    _: In<PlayerHandle>,
    btn: Res<Input<MouseButton>>,
    input: Res<Input<KeyCode>>,
    cursor: Res<Cursor>,
    player: Res<LocalPlayerHandle>,
    game_data: Res<GameData>,
) -> NetInput {
    let mut action = check_action(player, game_data);

    let mut pressed = Pack::default();
    let mut just_pressed = Pack::default();
    if btn.just_pressed(MouseButton::Left) {
        if action {
            just_pressed |= PLACE;
        } else {
            just_pressed |= INSPECT;
        }
    } else if input.pressed(KeyCode::Q) {
        pressed |= ACTION;
    } else if input.pressed(KeyCode::Escape) {
        pressed |= CAMERA;
    }
    let (grid_x, grid_y) = snap_to_grid(cursor.0);

    NetInput {
        pressed,
        just_pressed,
        grid_x: grid_x,
        grid_y: grid_y,
    }
}
