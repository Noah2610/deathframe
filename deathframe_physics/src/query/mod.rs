use crate::collision::prelude::*;
use crate::collision::tag::CollisionTag;
use crate::components::prelude::Collider;

enum CheckOperation {
    Any,
    All,
}

impl Default for CheckOperation {
    fn default() -> Self {
        CheckOperation::Any
    }
}

/// The `Query` can be used to check for collisions
/// on a `Collider`.
pub struct Query<'a, C>
where
    C: 'static + CollisionTag,
{
    collider:        &'a Collider<C>,
    check_states:    Vec<CollisionState<C>>,
    check_states_op: CheckOperation,
    return_op:       CheckOperation,
}

impl<'a, C> Query<'a, C>
where
    C: 'static + CollisionTag,
{
    /// Returns a new `Query` for the given `Collider`.
    pub fn new(collider: &'a Collider<C>) -> Self {
        Self {
            collider,
            check_states: Vec::new(),
            check_states_op: CheckOperation::default(),
            return_op: CheckOperation::default(),
        }
    }

    /// Query returns `true` if _any_ collisions matched.
    /// _(default)_
    pub fn any(mut self) -> Self {
        self.return_op = CheckOperation::Any;
        self
    }

    /// Runs the query against _all_ collisions,
    /// and returns `true` if _all_ matched.
    pub fn all(mut self) -> Self {
        self.return_op = CheckOperation::All;
        self
    }

    /// A collision's query passes, as long as _any_ given
    /// `CollisionState` (via `.state()`) matches each collision.
    /// _(default)_
    pub fn any_state(mut self) -> Self {
        self.check_states_op = CheckOperation::Any;
        self
    }

    /// A collision's query passes, only if _all_ given
    /// `CollisionState`s (via `.state()`) match each collision.
    pub fn all_states(mut self) -> Self {
        self.check_states_op = CheckOperation::All;
        self
    }

    /// Check collisions against given state.
    /// Multiple states checks can be added.
    pub fn state(mut self, state: CollisionState<C>) -> Self {
        self.check_states.push(state);
        self
    }

    /// Run the query.
    pub fn run(self) -> bool {
        let mut values = self.collider.collisions.values();
        let condition = |collision: &CollisionData<C>| {
            let check_state_condition = |state: &CollisionState<C>| match state
            {
                CollisionState::Enter(check_state_data) => {
                    if let CollisionState::Enter(state_data) = &collision.state
                    {
                        check_state_data.side == state_data.side
                            && check_state_data
                                .tag
                                .collides_with(&state_data.tag)
                    } else {
                        false
                    }
                }
                CollisionState::Steady(check_state_data) => {
                    if let CollisionState::Steady(state_data) = &collision.state
                    {
                        check_state_data.side == state_data.side
                            && check_state_data
                                .tag
                                .collides_with(&state_data.tag)
                    } else {
                        false
                    }
                }
                CollisionState::Leave => {
                    if let CollisionState::Leave = collision.state {
                        true
                    } else {
                        false
                    }
                }
            };

            match self.check_states_op {
                CheckOperation::Any => {
                    self.check_states.iter().any(check_state_condition)
                }
                CheckOperation::All => {
                    self.check_states.iter().all(check_state_condition)
                }
            }
        };
        match self.return_op {
            CheckOperation::Any => values.any(condition),
            CheckOperation::All => values.all(condition),
        }
    }
}
