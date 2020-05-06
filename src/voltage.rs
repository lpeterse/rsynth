use crate::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Voltage(pub f32);

impl<R: Rate> Signal<R> for Voltage {
    type Sample = Self;

    fn sample(&mut self) -> Self {
        Self(self.0)
    }
}
