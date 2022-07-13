use super::camera::*;
use super::game::*;
use super::map::*;
use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use ggrs::PlayerHandle;

pub type PressedPack = i32;

pub const PLACE: PressedPack = 1 << 4;
pub const CAMERA: PressedPack = 1 << 2;
pub const ACTION: PressedPack = 1 << 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct NetInput {
    pub pressed: PressedPack,
    pub grid_x: i32,
    pub grid_y: i32,
}

// TODO can define user mapped keys here
pub fn input_system(
    _: In<PlayerHandle>,
    input: Res<Input<KeyCode>>,
    cursor: Res<Cursor>,
    player: Res<LocalPlayerHandle>,
    game_data: Res<GameData>,
) -> NetInput {
    let mut action: bool;
    if (game_data.attacker == player.id && player.mode == Mode::Deploying)
        || (game_data.defender == player.id && player.mode == Mode::Building)
    {
        action = true;
    } else {
        action = false;
    }

    let mut pressed = PressedPack::default();

    if input.pressed(KeyCode::P) && action {
        pressed |= PLACE;
    } else if input.pressed(KeyCode::Q) {
        pressed |= ACTION;
    } else if input.pressed(KeyCode::Escape) {
        pressed |= CAMERA;
    }
    let (grid_x, grid_y) = snap_to_grid(cursor.0);

    NetInput {
        pressed,
        grid_x: grid_x,
        grid_y: grid_y,
    }
}
