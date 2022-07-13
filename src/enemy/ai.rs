use bevy::{math::Mat2, prelude::*};
use bevy_bobs::physics_2d::*;
use std::collections::HashMap;

use super::Enemy;

// dumb ai that attempts to move to target in straight line
#[derive(Component)]
pub struct DumbMoveAI {
    speed: f32,
    target: Option<Vec2>,
}

// ai with flocking behavior
#[derive(Component, Default, Clone)]
pub struct BoidMoveAI {
    pub view_angle: f32,
    pub view_range: f32,
    pub coherence: f32,       // weight for coherence
    pub alignment: f32,       // weight for alignment
    pub seperation: f32,      // weight for separation
    pub randomess: f32,       // weight for randomness
    pub tracking: f32,        // weight for tracking
    pub wander_angle: u32,    // range between 0..359
    pub target: Option<Vec2>, // optional target to move towards
}

#[derive(Component)]
pub struct RangeAttackAI {
    pub attack_range: f32, // min distance from target at which will begin to attack
                           // pub preference: AttackPreference,
}

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(dumb_ai_system).add_system(boid_ai_system);
    }
}

impl DumbMoveAI {
    pub fn new(speed: f32) -> Self {
        DumbMoveAI {
            speed,
            target: None,
        }
    }
    pub fn set_target(&mut self, target_pos: Vec2) {
        self.target = Some(target_pos);
    }
    pub fn unset_target(&mut self) {
        self.target = None;
    }
}

pub fn dumb_ai_system(time: Res<Time>, mut query: Query<(&mut Transform, &DumbMoveAI)>) {
    for (mut trans, ai) in query.iter_mut() {
        if ai.target.is_none() {
            continue;
        }

        let target_dir = (ai.target.unwrap() - trans.translation.truncate())
            .normalize_or_zero()
            .extend(0.);
        trans.translation += ai.speed * target_dir * time.delta_seconds();
    }
}

pub fn boid_ai_system(mut query: Query<(Entity, &mut Transform, &BoidMoveAI, &mut RigidBody)>) {
    let mut force_updates: HashMap<Entity, Vec2> = HashMap::new();
    for (self_entity, self_trans, self_ai, self_rb) in query.iter() {
        // fetch all boids in viewing range
        let mut neighbours: Vec<(Transform, BoidMoveAI, RigidBody)> = vec![];
        for (other_entity, other_trans, other_ai, other_rb) in query.iter() {
            if self_entity == other_entity {
                continue;
            }
            if self_trans.translation.distance(other_trans.translation) < self_ai.view_range {
                neighbours.push((other_trans.clone(), other_ai.clone(), other_rb.clone()));
            }
        }

        if neighbours.len() == 0 {
            continue;
        }

        let mut cur_force = force_updates
            .get(&self_entity)
            .unwrap_or(&Vec2::ZERO)
            .clone();

        // randomness force
        use rand::{thread_rng, Rng};
        use std::f32::consts::PI;

        let rand: i32 = thread_rng().gen_range(0..(self_ai.wander_angle as i32));
        let angle_deviation = ((rand - 180) as f32) * PI / 180.;
        let forward = self_rb.velocity.angle_between(Vec2::X);
        let random_force =
            (Mat2::from_angle(angle_deviation + forward) * Vec2::X) * self_ai.randomess;
        cur_force += random_force;

        // alignment (attempt to face same direction as neighbours)
        let avg_heading = neighbours
            .iter()
            .fold(Vec2::ZERO, |acc, (_, _, rb)| acc + rb.velocity)
            / neighbours.len() as f32;
        cur_force += avg_heading * self_ai.alignment + cur_force;

        // cohesion
        let avg_position = neighbours
            .iter()
            .fold(Vec3::ZERO, |acc, (trans, _, _)| acc + trans.translation)
            / neighbours.len() as f32;
        cur_force += (avg_position - self_trans.translation).truncate() * self_ai.coherence;

        // separation
        let seperation_force = neighbours.iter().fold(Vec2::ZERO, |acc, (trans, _, _)| {
            let dist = trans.translation.distance(self_trans.translation);
            let dir = (self_trans.translation - trans.translation).truncate();
            acc + dir / dist
        });
        cur_force += seperation_force * self_ai.seperation;

        // target
        if let Some(target) = self_ai.target {
            let target_force = target - self_trans.translation.truncate();
            cur_force += target_force * self_ai.tracking;
        }

        force_updates.insert(self_entity, cur_force);
    }

    // update all the forces

    for (e, _, ai, mut rb) in query.iter_mut() {
        if let Some(force) = force_updates.get(&e) {
            rb.force += *force * 5.;
        }
    }
}
