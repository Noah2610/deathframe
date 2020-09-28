use super::system_prelude::*;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

/// The `SwitchAnimationsSystem` handles entities'
/// `Animation`s with their `AnimationsContainer`s.
pub struct SwitchAnimationsSystem<K>
where
    K: Hash + Eq + Send + Sync + Debug + Clone,
{
    _k: PhantomData<K>,
}

impl<'a, K> System<'a> for SwitchAnimationsSystem<K>
where
    K: 'static + Hash + Eq + Send + Sync + Debug + Clone,
{
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, AnimationsContainer<K>>,
        WriteStorage<'a, Animation>,
        ReadStorage<'a, Unloaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut animations_containers,
            mut animations,
            unloaded_store,
        ): Self::SystemData,
    ) {
        for (entity, animations_container, _) in
            (&entities, &mut animations_containers, !&unloaded_store).join()
        {
            if let Some(existing_animation) = animations.get(entity) {
                if existing_animation.has_played_and_is_finished() {
                    match animations_container.pop() {
                        Ok(_) => (),
                        Err(e) => eprintln!(
                            "[WARNING]\n    First animation in \
                             AnimationsContainer's animations stack\n    \
                             should be an endlessly cycling animation\n    {}",
                            e
                        ),
                    }
                }
            }

            if animations_container.should_update {
                animations_container.should_update = false;
                if let Some(current_key) = animations_container.current() {
                    // Switch animation
                    self.play_animation(
                        entity,
                        current_key,
                        animations_container,
                        &mut animations,
                    );
                } else {
                    // Remove Animation component if there is no more animation to play
                    let _ = animations.remove(entity);
                }
            }
        }
    }
}

impl<K> SwitchAnimationsSystem<K>
where
    K: Hash + Eq + Send + Sync + Debug + Clone,
{
    fn play_animation(
        &mut self,
        entity: Entity,
        animation_key: &K,
        animations_container: &AnimationsContainer<K>,
        animations: &mut WriteStorage<Animation>,
    ) {
        if let Some(animation) = animations_container.animation(animation_key) {
            animations
                .insert(entity, animation)
                .expect("Couldn't insert Animation");
        } else {
            eprintln!(
                "[WARNING]\n    AnimationsContainer doesn't have the \
                 animation key \"{:?}\"",
                animation_key,
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
            _k: Default::default(),
        }
    }
}
