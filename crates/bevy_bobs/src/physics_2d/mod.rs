use bevy::prelude::*;

pub struct WorldGravity(pub Vec2);

#[derive(Component)]
pub struct RigidBody2D {
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
            .add_system(physics_2d_system)
            .add_system_to_stage(CoreStage::Last, physics_2d_reset_force_system);
    }
}

pub fn physics_2d_system(time: Res<Time>, mut query: Query<&mut RigidBody2D>) {
    for mut rb in query.iter_mut() {
        let force = rb.force;
        let mass = rb.mass;
        rb.velocity += force / mass * time.delta_seconds();
    }
}

fn physics_2d_reset_force_system(
    world_gravity: Res<WorldGravity>,
    mut query: Query<&mut RigidBody2D>,
) {
    for mut rb in query.iter_mut() {
        rb.force = if let Some(grav_override) = rb.gravity_scale {
            grav_override
        } else {
            world_gravity.0
        }
    }
}
