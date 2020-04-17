use super::CollisionState;
use std::cmp::PartialEq;
use std::hash::Hash;

#[derive(PartialEq, Eq, Clone, Hash, Deserialize)]
pub enum QueryValueState {
    Enter,
    EnterSide,
    EnterOrEnterSide,
    Steady,
    Leave,
}

impl PartialEq<CollisionState> for QueryValueState {
    fn eq(&self, coll_state: &CollisionState) -> bool {
        match (self, coll_state) {
            (QueryValueState::Enter, CollisionState::Enter(_)) => true,
            (QueryValueState::EnterSide, CollisionState::EnterSide(_)) => true,
            (QueryValueState::EnterOrEnterSide, CollisionState::Enter(_)) => {
                true
            }
            (
                QueryValueState::EnterOrEnterSide,
                CollisionState::EnterSide(_),
            ) => true,
            (QueryValueState::Steady, CollisionState::Steady(_)) => true,
            (QueryValueState::Leave, CollisionState::Leave) => true,
            (_, _) => false,
        }
    }
}
