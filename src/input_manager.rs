use std::collections::HashMap;

use amethyst::input::InputHandler;

#[derive(Clone, Copy, PartialEq)]
enum ActionState {
    Down,
    Up,
    Pressed,
    None,
}

impl Default for ActionState {
    fn default() -> Self {
        ActionState::None
    }
}

/// Manages input actions.
/// Stores data about which actions are _down_, _up_, or being _pressed_.
pub struct InputManager {
    actions: HashMap<String, ActionState>,
}

impl InputManager {
    /// Creates a new `InputManager`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if the action with the given name was pressed _down_.
    pub fn is_down<T>(&self, action: T) -> bool
    where
        T: ToString,
    {
        self.is_action_in_state(action, ActionState::Down)
    }

    /// Returns `true` if the action with the given name was released (_down_).
    pub fn is_up<T>(&self, action: T) -> bool
    where
        T: ToString,
    {
        self.is_action_in_state(action, ActionState::Up)
    }

    /// Returns `true` if the action with the given name is being _pressed_ down.
    pub fn is_pressed<T>(&self, action: T) -> bool
    where
        T: ToString,
    {
        self.is_action_in_state(action, ActionState::Pressed)
    }

    fn is_action_in_state<T>(&self, action: T, state: ActionState) -> bool
    where
        T: ToString,
    {
        if let Some(s) = self.actions.get(&action.to_string()) {
            s == &state
        } else {
            false
        }
    }

    /// This method is called every frame, by the `InputManagerSystem`.
    pub fn update(&mut self, input: &InputHandler<String, String>) {
        for action in input.bindings.actions() {
            let state = self
                .actions
                .entry(action.clone())
                .or_insert(ActionState::default());
            if let Some(is_down) = input.action_is_down(&action) {
                if is_down {
                    // IS DOWN
                    *state = match state {
                        // Was previously `Down` or `Pressed`, becomes or stays `Pressed`, as it is still pressed down.
                        ActionState::Down | ActionState::Pressed => {
                            ActionState::Pressed
                        }
                        // Was previously `Up` or `None`, becomes `Down`, as it is now newly pressed.
                        ActionState::Up | ActionState::None => {
                            ActionState::Down
                        }
                    };
                } else {
                    // NOT DOWN
                    *state = match state {
                        // Was previously `Down` or `Pressed`, becomes `Up`, as it is no longer pressed.
                        ActionState::Down | ActionState::Pressed => {
                            ActionState::Up
                        }
                        // Was previously `Up` or `None`, becomes or stays `None`.
                        ActionState::Up | ActionState::None => {
                            ActionState::None
                        }
                    };
                }
            } else {
                panic!(format!("Action should exist: {}", action));
            }
        }
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self {
            actions: HashMap::new(),
        }
    }
}
