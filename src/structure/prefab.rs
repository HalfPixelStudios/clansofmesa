use bevy_bobs::prefab::{models::*, *};
use serde::Deserialize;

pub enum AttackPreference {
    Strongest,
    Weakest,
    Furthest,
    Closest,
    Random,
    Enemy(String),
}

#[derive(Deserialize, Clone)]
pub struct TowerPrefab {
    pub display_name: Option<String>,
    pub health: u32,
    pub cost: u32,
    pub sprite_index: usize,
    pub sprite_color: ColorRGB,
}
