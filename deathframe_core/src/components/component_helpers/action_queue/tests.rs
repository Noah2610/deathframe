use super::ActionQueue;

type MockAction = u8;

#[derive(Default)]
struct MockQueue {
    actions: Vec<MockAction>,
}

impl ActionQueue for MockQueue {
    type Action = MockAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}

#[test]
fn can_add_actions() {
    let mut queue = MockQueue::default();
    queue.add_action(0);
    queue.add_action(1);
    queue.add_action(2);

    let actions = queue.actions;
    assert_eq!(actions, vec![0, 1, 2], "Should add actions in order");
}

#[test]
fn can_drain_actions() {
    let mut queue = MockQueue::default();
    queue.add_action(0);
    queue.add_action(1);

    for (i, action) in queue.drain_actions().enumerate() {
        assert_eq!(action, i as MockAction, "Should drain proper action");
    }

    for _ in queue.drain_actions() {
        panic!(
            "There should be no more actions left, they should all be drained"
        );
    }

    queue.add_action(10);
    for action in queue.drain_actions() {
        assert_eq!(
            action, 10 as MockAction,
            "Should drain final action, after adding it"
        );
    }
}
