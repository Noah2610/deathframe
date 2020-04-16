use super::system_prelude::*;

#[derive(Default)]
pub struct UpdateHealthSystem;

impl<'a> System<'a> for UpdateHealthSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, HealthActionQueue>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut health_store,
            mut health_action_queue_store,
        ): Self::SystemData,
    ) {
        for (_entity, health, health_action_queue) in
            (&entities, &mut health_store, &mut health_action_queue_store)
                .join()
        {
            for action in health_action_queue.drain_actions() {
                match action {
                    HealthAction::Gain(hp) => {
                        health.gain(hp);
                    }
                    HealthAction::Lose(hp) => {
                        health.lose(hp);
                    }
                }
            }
        }
    }
}
