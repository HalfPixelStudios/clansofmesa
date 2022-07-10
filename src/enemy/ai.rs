use bevy::prelude::*;

// dumb ai that attempts to move to target in straight line
#[derive(Component)]
pub struct DumbAI {
    pub speed: f32,
    pub target: Vec2,
}

pub fn dumb_ai_system(time: Res<Time>, mut query: Query<(&mut Transform, &DumbAI)>) {
    for (mut trans, ai) in query.iter_mut() {
        let target_dir = (ai.target - trans.translation.truncate())
            .normalize_or_zero()
            .extend(0.);
        trans.translation += ai.speed * target_dir * time.delta_seconds();
    }
}

// ai with flocking behavior
#[derive(Component, Clone)]
pub struct BoidAI {
    pub speed: f32,
    pub view_angle: f32,
    pub view_range: f32,
    pub coherence: f32,
    pub steering: f32,
    pub alignment: f32,
    heading: Vec2, // direction currently travelling in
}

pub fn boid_ai_system(mut query: Query<(Entity, &mut Transform, &BoidAI)>) {

    /*
    for (self_entity, self_trans, self_ai) in query.iter() {

        // fetch all boids in viewing range
        let mut neighbours: Vec<(Transform, BoidAI)> = vec!();
        for (other_entity, other_trans, other_ai) in query.iter() {
            if self_entity == other_entity {
                continue;
            }
            if self_trans.translation.distance(other_trans.translation) < self_ai.view_range {
                neighbours.push((other_trans.clone(), other_ai.clone()));
            }
        }

        // alignment (attempt to face same direction as neighbours)
        if let Some(avg_heading) = neighbours.iter().fold(Vec2::ZERO, |acc, b| acc + b.1.heading).try_normalize() {
            // self_ai.heading
        }

    }
    */
}
