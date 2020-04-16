pub mod prelude {
    pub use super::deals_damage::DealsDamage;
    pub use super::takes_damage::TakesDamage;
}

mod deals_damage;
mod takes_damage;

use super::component_prelude;
