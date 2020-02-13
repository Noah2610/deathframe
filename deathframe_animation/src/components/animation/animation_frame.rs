/// An `AnimationFrame` holds information on the _sprite\_id_ for this frame,
/// and the _duration\_ms_ for how long this frame should be rendered.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct AnimationFrame {
    pub(crate) sprite_id:   usize,
    pub(crate) duration_ms: u64,
}

impl AnimationFrame {
    pub fn builder() -> AnimationFrameBuilder {
        AnimationFrameBuilder::default()
    }
}

impl<T, U> From<(T, U)> for AnimationFrame
where
    T: Into<usize>,
    U: Into<u64>,
{
    fn from(data: (T, U)) -> Self {
        Self {
            sprite_id:   data.0.into(),
            duration_ms: data.1.into(),
        }
    }
}
