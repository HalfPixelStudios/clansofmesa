use bevy::prelude::*;

pub struct WorldGravity(pub Vec2);

#[derive(Component)]
pub struct RigidBody {
    pub mass: f32,
    pub gravity_scale: Option<Vec2>, // override the world's gravity
    pub linear_damping: f32,
    pub velocity: Vec2,
    pub force: Vec2,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldGravity(Vec2::ZERO))
            .add_system(physics_system)
            .add_system_to_stage(CoreStage::Last, reset_force_system);
    }
}

pub fn physics_system(time: Res<Time>, mut query: Query<&mut RigidBody>) {
    for mut rb in query.iter_mut() {
        let force = rb.force;
        let mass = rb.mass;
        rb.velocity += force / mass * time.delta_seconds();
    }
}

fn reset_force_system(world_gravity: Res<WorldGravity>, mut query: Query<&mut RigidBody>) {
    for mut rb in query.iter_mut() {
        rb.force = if let Some(grav_override) = rb.gravity_scale {
            grav_override
        } else {
            world_gravity.0
        }
    }
}
