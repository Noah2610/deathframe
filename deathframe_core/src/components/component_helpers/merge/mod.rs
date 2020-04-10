/// Merge types together.
pub trait Merge: Sized {
    /// Merge other value into self.
    fn merge(&mut self, other: Self);

    /// Consumes both values, merges them together,
    /// and returns a new instance of `Self`.
    fn merged(mut self, other: Self) -> Self {
        self.merge(other);
        self
    }
}

impl<T> Merge for Option<T>
where
    T: Merge,
{
    fn merge(&mut self, other: Self) {
        match self.as_mut() {
            Some(s) => match other {
                Some(o) => s.merge(o),
                None => (),
            },
            None => match other {
                Some(o) => *self = Some(o),
                None => (),
            },
        }
    }
}

#[cfg(test)]
mod tests;
