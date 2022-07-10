use bevy_bobs::prefab::models::ColorRGB;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Lifetimes {
    pub distance: Option<f32>,
    pub duration: Option<f32>,
    pub penetration: Option<u32>,
}

#[derive(Deserialize, Clone)]
pub struct BulletPrefab {
    pub display_name: Option<String>,
    pub damage: u32,
    pub speed: f32,
    pub lifetimes: Lifetimes,
    pub sprite_index: usize,
    pub sprite_color: ColorRGB,
}
