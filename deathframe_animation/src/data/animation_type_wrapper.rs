use crate::components::prelude::Animation;

/// Wraps an `Animation`, specifying its `AnimationFramesIter` variant
/// when turning this enum into an `Animation`.
pub enum AnimationTypeWrapper<A>
where
    A: Into<Animation>,
{
    /// Will make the `Animation` cycle endlessly,
    /// when turned into one with `.into()`.
    Cycle(A),
    /// Will make the `Animation` play once,
    /// when turned into one with `.into()`.
    Once(A),
}

impl<A> Into<Animation> for AnimationTypeWrapper<A>
where
    A: Into<Animation>,
{
    fn into(self) -> Animation {
        match self {
            AnimationTypeWrapper::Cycle(a) => {
                let mut anim = a.into();
                anim.play_cycle();
                anim
            }
            AnimationTypeWrapper::Once(a) => {
                let mut anim = a.into();
                anim.play_once();
                anim
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_animation() {
        let anim = vec![(0_u8, 100_u32), (1_u8, 100_u32), (2_u8, 100_u32)];
        let cycle = AnimationTypeWrapper::Cycle(anim.clone());
        let _: Animation = cycle.into();
        let once = AnimationTypeWrapper::Once(anim);
        let _: Animation = once.into();
    }
}
