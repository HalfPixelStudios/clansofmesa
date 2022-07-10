use super::camera::*;
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

    NetInput {
        pressed,
        grid_x: cursor.0.x as i32,
        grid_y: cursor.0.y as i32,
    }
}
