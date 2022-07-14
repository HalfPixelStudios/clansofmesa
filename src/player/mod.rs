pub mod place;

use bevy::prelude::*;

#[derive(Component)]
pub struct Inventory {
    gems: u32,
}

impl Inventory {
    pub fn add_gems(&mut self, amount: u32) {
        self.gems += amount;
    }
    pub fn consume_gems(&mut self, amount: u32) -> Option<u32> {
        if amount > self.gems {
            return None;
        }
        self.gems -= amount;
        Some(self.gems)
    }
}
