use super::component_prelude::*;
use super::health_action::HealthAction;

/// Can manipulate the entity's `Health` component via `HealthAction`s.
/// Through this component, the entity can _lose_ and _gain_ health.
#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct HealthActionQueue {
    actions: Vec<HealthAction>,
}

impl HealthActionQueue {
    pub fn gain(&mut self, hitpoints: HitPoints) {
        self.add_action(HealthAction::Gain(hitpoints))
    }

    pub fn lose(&mut self, hitpoints: HitPoints) {
        self.add_action(HealthAction::Lose(hitpoints))
    }
}

impl ActionQueue for HealthActionQueue {
    type Action = HealthAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
