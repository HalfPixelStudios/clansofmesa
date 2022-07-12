use bevy::prelude::*;
use bevy_bobs::physics_2d::*;

// dumb ai that attempts to move to target in straight line
#[derive(Component)]
pub struct DumbMoveAI {
    speed: f32,
    target: Option<Vec2>,
}

// ai with flocking behavior
#[derive(Component, Default, Clone)]
pub struct BoidMoveAI {
    pub speed: f32,
    pub view_angle: f32,
    pub view_range: f32,
    pub coherence: f32,
    pub steering: f32,
    pub alignment: f32,
    pub chaos: u32,           // randomness (lower is more random)
    pub target: Option<Vec2>, // optional target to move towards
}

pub enum AttackPreference {
    Strongest,
    Weakest,
    Furthest,
    Closest,
    Random,
}

#[derive(Component)]
pub struct RangeAttackAI {
    pub attack_range: f32, // min distance from target at which will begin to attack
    pub preference: AttackPreference,
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

pub fn boid_ai_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &BoidMoveAI, &RigidBody)>,
) {
    let mut heading_updates: Vec<(Entity, Vec2)> = vec![];
    for (self_entity, self_trans, self_ai, self_rb) in query.iter() {
        // fetch all boids in viewing range
        let mut neighbours: Vec<(Transform, BoidMoveAI)> = vec![];
        for (other_entity, other_trans, other_ai, other_rb) in query.iter() {
            if self_entity == other_entity {
                continue;
            }
            if self_trans.translation.distance(other_trans.translation) < self_ai.view_range {
                neighbours.push((other_trans.clone(), other_ai.clone()));
            }
        }

        // alignment (attempt to face same direction as neighbours)
        if let Some(avg_heading) = neighbours
            .iter()
            .fold(Vec2::ZERO, |acc, b| acc + b.1.heading)
            .try_normalize()
        {
            // self_ai.heading
        }

        // cohesion
        if let Some(avg_position) = neighbours
            .iter()
            .fold(Vec3::ZERO, |acc, b| acc + b.0.translation)
            .try_normalize()
        {}

        // target
    }

    // move
    for (_, mut transform, ai) in query.iter_mut() {
        transform.translation += ai.speed * ai.heading.extend(0.) * time.delta_seconds();
    }
}
