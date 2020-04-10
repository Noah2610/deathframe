/// The `ActionQueue` can receive and accumulate actions,
/// which can then be consumed at some point.
pub trait ActionQueue {
    type Action;

    /// Returns a mutable reference to the `Vec` of actions `Action`.
    fn mut_actions(&mut self) -> &mut Vec<Self::Action>;

    /// Add an `Action` to the action queue.
    fn add_action(&mut self, action: Self::Action) {
        self.mut_actions().push(action);
    }

    /// Returns a draining iterator over all queued actions.
    /// Consume the actions in the queue.
    fn drain_actions(&mut self) -> std::vec::Drain<Self::Action> {
        self.mut_actions().drain(..)
    }
}

#[cfg(test)]
mod tests;
