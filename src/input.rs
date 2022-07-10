use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use ggrs::PlayerHandle;

pub type PressedPack = u32;

pub const INPUT_UP: PressedPack = 1 << 0;
pub const INPUT_DOWN: PressedPack = 1 << 1;
pub const INPUT_LEFT: PressedPack = 1 << 2;
pub const INPUT_RIGHT: PressedPack = 1 << 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct NetInput {
    pub pressed: PressedPack,
}

// TODO can define user mapped keys here
pub fn input_system(_: In<PlayerHandle>, input: Res<Input<KeyCode>>) -> PressedPack {
    let mut pressed = PressedPack::default();

    if input.pressed(KeyCode::W) {
        pressed |= INPUT_UP;
    }
    if input.pressed(KeyCode::S) {
        pressed |= INPUT_DOWN;
    }
    if input.pressed(KeyCode::A) {
        pressed |= INPUT_LEFT;
    }
    if input.pressed(KeyCode::D) {
        pressed |= INPUT_RIGHT;
    }

    // NetInput { pressed }
    pressed
}
