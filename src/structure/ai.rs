use bevy::prelude::*;
use bevy_bobs::component::health::Health;
use std::cmp::Ordering;

use crate::enemy::Enemy;

use super::{prefab::AttackPreference, Tower};

#[derive(Component)]
pub struct AttackAI {
    pub preference: AttackPreference,
}

pub fn attack_system(
    tower_query: Query<(&AttackAI, &Transform), With<Tower>>,
    enemy_query: Query<(Entity, &Transform, &Health), With<Enemy>>,
) {
    for (ai, trans) in tower_query.iter() {
        let target: Option<(Entity, &Transform)> = match ai.preference {
            AttackPreference::Closest => enemy_query
                .iter()
                .min_by(|(_, x, _), (_, y, _)| {
                    x.translation
                        .distance(trans.translation)
                        .total_cmp(&y.translation.distance(trans.translation))
                })
                .map(|(e, trans, _)| (e, trans)),
            AttackPreference::Furthest => enemy_query
                .iter()
                .max_by(|(_, x, _), (_, y, _)| {
                    x.translation
                        .distance(trans.translation)
                        .total_cmp(&y.translation.distance(trans.translation))
                })
                .map(|(e, trans, _)| (e, trans)),
            AttackPreference::Weakest => enemy_query
                .iter()
                .min_by(|(_, _, x), (_, _, y)| x.cmp(y))
                .map(|(e, trans, _)| (e, trans)),
            AttackPreference::Strongest => enemy_query
                .iter()
                .max_by(|(_, _, x), (_, _, y)| x.cmp(y))
                .map(|(e, trans, _)| (e, trans)),
            _ => None,
        };

        if let Some((target_entity, target_trans)) = target {}
    }
}
