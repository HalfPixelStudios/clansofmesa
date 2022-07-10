use bevy::prelude::*;
use std::time::Duration;

use crate::misc::displacement::*;

use super::health::Health;

pub trait Lifetime {
    fn is_expired(&self) -> bool;
    fn reset(&mut self);
}

#[derive(Component)]
pub struct DistanceLifetime {
    max_distance: f32,
    displacement: Displacement,
    expired: bool,
}

#[derive(Component)]
pub struct DurationLifetime {
    timer: Timer,
    expired: bool,
}

#[derive(Component)]
pub struct PenetrationLifetime {
    health: Health,
}

impl DistanceLifetime {
    pub fn new(max_distance: f32) -> Self {
        DistanceLifetime {
            max_distance,
            displacement: Displacement::new(),
            expired: false,
        }
    }
}

impl Lifetime for DistanceLifetime {
    fn is_expired(&self) -> bool {
        self.expired
    }
    fn reset(&mut self) {
        self.expired = false;
    }
}

impl DurationLifetime {
    pub fn new(max_duration: f32) -> Self {
        DurationLifetime {
            timer: Timer::new(Duration::from_millis((max_duration * 1000.) as u64), false),
            expired: false,
        }
    }
}

impl Lifetime for DurationLifetime {
    fn is_expired(&self) -> bool {
        self.expired
    }
    fn reset(&mut self) {
        self.expired = false;
    }
}

impl PenetrationLifetime {
    pub fn new(penetration: u32) -> Self {
        PenetrationLifetime {
            health: Health::new(penetration),
        }
    }
    pub fn tick(&mut self) {
        self.health.take(1);
    }
}

impl Lifetime for PenetrationLifetime {
    fn is_expired(&self) -> bool {
        self.health.is_zero()
    }
    fn reset(&mut self) {
        self.health.reset()
    }
}

pub fn duration_lifetime_system(time: Res<Time>, mut query: Query<&mut DurationLifetime>) {
    for mut lifetime in query.iter_mut() {
        if lifetime.expired {
            continue;
        }

        lifetime.timer.tick(time.delta());

        if lifetime.timer.finished() {
            lifetime.expired = true;
        }
    }
}

pub fn distance_lifetime_system(mut query: Query<(&Transform, &mut DistanceLifetime)>) {
    for (transform, mut lifetime) in query.iter_mut() {
        if lifetime.expired {
            continue;
        }

        if lifetime.displacement.get_distance() > lifetime.max_distance {
            lifetime.expired = true;
        }

        lifetime.displacement.update(transform.translation);
    }
}
