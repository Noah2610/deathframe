use super::HitPoints;

#[derive(Clone, Deserialize)]
pub enum HealthAction {
    /// _Gain_ health.
    Gain(HitPoints),
    /// _Lose_ health.
    Lose(HitPoints),
}
