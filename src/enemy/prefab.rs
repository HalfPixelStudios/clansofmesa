use bevy_bobs::prefab::{models::*, *};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub enum AI {
    Dumb {
        speed: f32,
    },
    Boid {
        view_angle: f32,
        view_range: f32,
        coherence: f32,
        alignment: f32,
        seperation: f32,
        randomess: f32,
        tracking: f32,
        wander_angle: u32,
    },
}

#[derive(Deserialize, Clone)]
pub struct EnemyPrefab {
    pub display_name: Option<String>,
    pub health: u32,
    pub reward: u32,
    pub ai: AI,
    pub sprite_index: usize,
    pub sprite_color: ColorRGB,
}
