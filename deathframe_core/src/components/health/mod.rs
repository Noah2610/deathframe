pub mod prelude {
    pub use super::health_action::HealthAction;
    pub use super::health_action_queue::HealthActionQueue;
    pub use super::Health;
    pub use super::HitPoints;
}

mod health_action;
mod health_action_queue;

use super::component_prelude::{self, *};

pub type HitPoints = u32;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields)]
pub struct Health {
    pub health:     HitPoints,
    pub max_health: HitPoints,
}

impl Health {
    /// Gain the given amount of hitpoints, without exceeding its max health.
    pub fn gain(&mut self, hp: HitPoints) {
        self.health = (self.health + hp).min(self.max_health);
    }

    /// Lose the given amount of hitpoints, wihout going below 0.
    pub fn lose(&mut self, hp: HitPoints) {
        self.health = self.health.checked_sub(hp).unwrap_or(0);
    }

    /// Checks if health is above 0.
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    /// Checks if the health is at max health.
    pub fn has_full_health(&self) -> bool {
        self.health == self.max_health
    }
}
