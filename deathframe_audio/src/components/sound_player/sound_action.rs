use std::hash::Hash;

#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[derive(Clone)]
pub enum SoundAction<K>
where
    K: PartialEq + Eq + Hash,
{
    /// Play the sound associated to the given sound key `K`.
    /// Sound is played with the default volume level.
    Play(K),
    /// Plays the given sound key `K`'s sound with
    /// the given volume level (`0.0` - `1.0`).
    PlayWithVolume(K, f32),
}
