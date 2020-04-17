/// A lifecycle's state.
/// Entities' live in variant declaration order, meaning they
/// first have the `Initial` state, and end with the `Despawn` state.
///
/// Some `LifecycleState` variants can be _prolonged_.
/// Prolonging means, their current state will be active for
/// at least N more frames. See the `Lifecycle::prolong` function.
#[derive(Clone, Deserialize, PartialEq, Eq, Hash)]
pub enum LifecycleState {
    /// Default state, switches to `Spawn` ASAP.
    /// Prefer checking for `Spawn` state in game code.
    Initial,
    /// Stays in this state for at least one frame, after it was created.
    /// Can be _prolonged_.
    Spawn,
    /// Stays in this state until the entity's `Health`
    /// says it's dead. If the entity has no `Health` component,
    /// then the entity will stay in this state indefinitely.
    Alive,
    /// When `Health` determines the entity is dead, then
    /// this is the new state for at least one frame.
    /// Can be _prolonged_.
    Death,
    /// After `Death`, switches to `Despawn`.
    /// `Despawn` means this entity will be deleted ASAP.
    /// Prefer checking for `Death` state in game code.
    Despawn,
}

impl Default for LifecycleState {
    fn default() -> Self {
        LifecycleState::Initial
    }
}
