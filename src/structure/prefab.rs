use bevy_bobs::{
    attack_pattern::AttackPattern,
    prefab::{models::*, *},
};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub enum AttackPreference {
    Strongest,
    Weakest,
    Furthest,
    Closest,
    Random,
    Enemy(String),
}

#[derive(Deserialize, Clone)]
pub struct SimpleAttackAI {
    pub bullet_id: PrefabId,
    pub preference: AttackPreference,
    pub attack_range: f32,
    pub attack_pattern: AttackPattern,
}

#[derive(Deserialize, Clone)]
pub struct TowerPrefab {
    pub display_name: Option<String>,
    pub health: u32,
    pub cost: u32,
    pub ai: SimpleAttackAI,
    pub sprite_index: usize,
    pub sprite_color: ColorRGB,
}
