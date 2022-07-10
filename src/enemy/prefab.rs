use bevy_bobs::prefab::{models::*, *};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct EnemyPrefab {
    pub display_name: Option<String>,
    pub health: u32,
    pub reward: u32,
    pub sprite_index: usize,
    pub sprite_color: ColorRGB,
}
