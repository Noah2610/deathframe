use super::system_prelude::*;

/// Handles updating of entity's `Lifecycle` component's `LifecycleState`.
/// Will also delete entities when their state switches to `Despawn`.
#[derive(Default)]
pub struct UpdateLifecycleSystem;

impl<'a> System<'a> for UpdateLifecycleSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Lifecycle>,
        ReadStorage<'a, Health>,
    );

    fn run(
        &mut self,
        (entities, mut lifecycle_store, health_store): Self::SystemData,
    ) {
        for (entity, lifecycle, health_opt) in
            (&entities, &mut lifecycle_store, health_store.maybe()).join()
        {
            if !lifecycle.is_prolonged() {
                match &lifecycle.state {
                    LifecycleState::Initial => lifecycle.next_state().unwrap(),
                    LifecycleState::Spawn => {
                        lifecycle.next_state().unwrap();
                    }
                    LifecycleState::Alive => {
                        if let Some(health) = health_opt {
                            if !health.is_alive() {
                                lifecycle.next_state().unwrap();
                            }
                        }
                    }
                    LifecycleState::Death => {
                        lifecycle.next_state().unwrap();
                    }
                    LifecycleState::Despawn => {
                        entities.delete(entity).expect(
                            "Couldn't delete entity with \
                             LifecycleState::Despawn",
                        );
                    }
                }
            }
            lifecycle.update();
        }
    }
}
