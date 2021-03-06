use bevy::{core::Stopwatch, prelude::*};
use bevy_bobs::{
    attack_pattern::{self, *},
    component::health::Health,
    prefab::PrefabId,
};

use crate::{bullet::SpawnBulletEvent, enemy::Enemy};

use super::{
    prefab::{AttackPreference, SimpleAttackAI},
    Tower,
};

#[derive(Component, Clone)]
pub struct AttackAI {
    pub bullet_id: PrefabId,
    pub preference: AttackPreference,
    pub attack_range: f32,
    pub attack_speed: f32,
    pub attack_pattern: AttackPattern,
    attack_timer: Stopwatch,
}

impl From<SimpleAttackAI> for AttackAI {
    fn from(ai: SimpleAttackAI) -> Self {
        let SimpleAttackAI {
            bullet_id,
            preference,
            attack_range,
            attack_speed,
            attack_pattern,
        } = ai;
        AttackAI {
            bullet_id,
            preference,
            attack_range,
            attack_speed,
            attack_pattern,
            attack_timer: Stopwatch::new(),
        }
    }
}

pub fn attack_system(
    time: Res<Time>,
    mut tower_query: Query<(&mut AttackAI, &Transform), With<Tower>>,
    enemy_query: Query<(Entity, &Transform, &Health), With<Enemy>>,
    mut writer: EventWriter<SpawnBulletEvent>,
) {
    for (mut ai, trans) in tower_query.iter_mut() {
        ai.attack_timer.tick(time.delta());

        if ai.attack_timer.elapsed_secs() < ai.attack_speed {
            continue;
        }
        ai.attack_timer.reset();

        let visible_enemies = enemy_query.iter().filter(|(_, enemy_trans, _)| {
            trans.translation.distance(enemy_trans.translation) < ai.attack_range
        });
        let target: Option<(Entity, &Transform)> = match ai.preference {
            AttackPreference::Closest => visible_enemies
                .min_by(|(_, x, _), (_, y, _)| {
                    x.translation
                        .distance(trans.translation)
                        .total_cmp(&y.translation.distance(trans.translation))
                })
                .map(|(e, trans, _)| (e, trans)),
            AttackPreference::Furthest => visible_enemies
                .max_by(|(_, x, _), (_, y, _)| {
                    x.translation
                        .distance(trans.translation)
                        .total_cmp(&y.translation.distance(trans.translation))
                })
                .map(|(e, trans, _)| (e, trans)),
            AttackPreference::Weakest => visible_enemies
                .min_by(|(_, _, x), (_, _, y)| x.cmp(y))
                .map(|(e, trans, _)| (e, trans)),
            AttackPreference::Strongest => visible_enemies
                .max_by(|(_, _, x), (_, _, y)| x.cmp(y))
                .map(|(e, trans, _)| (e, trans)),
            _ => None,
        };

        if let Some((_, target_trans)) = target {
            let dir = (target_trans.translation - trans.translation)
                .truncate()
                .normalize_or_zero();
            let attack_dirs = generate_attack_points(dir, &ai.attack_pattern);

            for attack_dir in attack_dirs.iter() {
                writer.send(SpawnBulletEvent {
                    id: ai.bullet_id.clone(),
                    spawn_pos: trans.translation.truncate(),
                    dir: *attack_dir,
                });
            }
        };
    }
}
