use super::camera::*;
use super::map::*;
use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use ggrs::PlayerHandle;

pub type PressedPack = i32;

pub const PLACE: PressedPack = 1 << 4;

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
) -> NetInput {
    let mut pressed = PressedPack::default();

    if input.pressed(KeyCode::P) {
        info!("{:?}", PLACE);
        pressed |= PLACE;
    }
    let (grid_x, grid_y) = snap_to_grid(cursor.0);

    NetInput {
        pressed,
        grid_x: grid_x - 8,
        grid_y: grid_y - 8,
    }
}
