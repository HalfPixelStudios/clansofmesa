use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use ggrs::PlayerHandle;

pub type InputPack = u32;

pub const INPUT_UP: InputPack = 1 << 0;
pub const INPUT_DOWN: InputPack = 1 << 1;
pub const INPUT_LEFT: InputPack = 1 << 2;
pub const INPUT_RIGHT: InputPack = 1 << 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct GGRSInput {
    pub keyboard: InputPack,
}

// TODO can define user mapped keys here
pub fn pack_input(_: In<PlayerHandle>, input: Res<Input<KeyCode>>) -> InputPack {
    let mut input_pack = InputPack::default();

    if input.pressed(KeyCode::W) {
        input_pack |= INPUT_UP;
    }
    if input.pressed(KeyCode::S) {
        input_pack |= INPUT_DOWN;
    }
    if input.pressed(KeyCode::A) {
        input_pack |= INPUT_LEFT;
    }
    if input.pressed(KeyCode::D) {
        input_pack |= INPUT_RIGHT;
    }

    input_pack
}
