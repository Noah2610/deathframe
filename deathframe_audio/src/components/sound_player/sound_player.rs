use super::component_prelude::*;
use super::sound_action::SoundAction;
use std::hash::Hash;

#[derive(Component)]
#[storage(VecStorage)]
pub struct SoundPlayer<K>
where
    K: 'static + PartialEq + Eq + Hash + Send + Sync,
{
    actions: Vec<SoundAction<K>>,
}

impl<K> ActionQueue for SoundPlayer<K>
where
    K: PartialEq + Eq + Hash + Send + Sync,
{
    type Action = SoundAction<K>;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}

impl<K> Default for SoundPlayer<K>
where
    K: PartialEq + Eq + Hash + Send + Sync,
{
    fn default() -> Self {
        Self {
            actions: Vec::new(),
        }
    }
}
