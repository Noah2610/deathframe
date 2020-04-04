use super::system_prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// The `SwitchAnimationsSystem` handles entities'
/// `Animation`s with their `AnimationsContainer`s.
pub struct SwitchAnimationsSystem<K>
where
    K: Hash + Eq + Send + Sync + Debug + Clone,
{
    entity_animations: HashMap<Entity, K>,
}

impl<'a, K> System<'a> for SwitchAnimationsSystem<K>
where
    K: 'static + Hash + Eq + Send + Sync + Debug + Clone,
{
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, AnimationsContainer<K>>,
        WriteStorage<'a, Animation>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            animations_containers,
            mut animations,
            loadables,
            loadeds,
        ): Self::SystemData,
    ) {
        let mut entity_animations = HashMap::new();

        for (entity, animations_container) in
            (&entities, &animations_containers)
                .join()
                .filter(|(entity, _)| {
                    is_entity_loaded(*entity, &loadables, &loadeds)
                })
        {
            if let Some(current_key) = animations_container.current() {
                entity_animations.insert(entity, current_key.clone());
                // An animation should be playing
                if let Some(saved_playing_key) =
                    self.entity_animations.get(&entity).cloned()
                {
                    // Switch animation
                    if current_key != &saved_playing_key {
                        self.play_current_animation(
                            entity,
                            animations_container,
                            &mut animations,
                        );
                    }
                } else {
                    // Insert initial animation
                    self.play_current_animation(
                        entity,
                        animations_container,
                        &mut animations,
                    );
                }
            }
        }

        self.entity_animations = entity_animations;
    }
}

impl<K> SwitchAnimationsSystem<K>
where
    K: Hash + Eq + Send + Sync + Debug + Clone,
{
    fn play_current_animation(
        &mut self,
        entity: Entity,
        animations_container: &AnimationsContainer<K>,
        animations: &mut WriteStorage<Animation>,
    ) {
        if let Some(animation) = animations_container.current_animation() {
            animations
                .insert(entity, animation)
                .expect("Couldn't insert Animation");
        } else {
            eprintln!(
                "WARNING: AnimationsContainer doesn't have a current animation",
            );
        }
    }
}

impl<K> Default for SwitchAnimationsSystem<K>
where
    K: Hash + Eq + Send + Sync + Debug + Clone,
{
    fn default() -> Self {
        Self {
            entity_animations: Default::default(),
        }
    }
}
