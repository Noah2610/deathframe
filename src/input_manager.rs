use std::collections::HashMap;

use amethyst::input::InputHandler;

type AxisValue = f64;

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
#[derive(Default)]
pub struct InputManager {
    actions: HashMap<String, ActionState>,
    axes:    HashMap<String, AxisValue>,
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

    /// Behaves identically to `amethyst::input::InputHandler::axis_value`.
    pub fn axis_value<T>(&self, axis: T) -> Option<AxisValue>
    where
        T: ToString,
    {
        self.axes.get(&axis.to_string()).map(Clone::clone)
    }

    /// Similar to `axis_value`, but instead of passing a specific axis string ID,
    /// pass a function, which is called with every registered axis ID and value; the function returns a boolean;
    /// when the function returns `true`, then return the axis value of that axis.
    pub fn axis_value_find<F>(&self, find_func: F) -> Option<AxisValue>
    where
        F: Fn(&(&String, &AxisValue)) -> bool,
    {
        self.axes.iter().find(find_func).map(|(_, value)| *value)
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
        self.update_actions(input);
        self.update_axes(input);
    }

    fn update_actions(&mut self, input: &InputHandler<String, String>) {
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

    fn update_axes(&mut self, input: &InputHandler<String, String>) {
        for axis in input.bindings.axes() {
            if let Some(value) = input.axis_value(&axis) {
                let entry = self.axes.entry(axis).or_insert(0.0);
                if *entry != value {
                    *entry = value;
                }
            } else {
                panic!(format!("Axis should exist: {}", axis));
            }
        }
    }
}
