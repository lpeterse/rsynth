use crate::*;
use std::fmt::Debug;

pub trait Factor: Clone + Copy + Debug + Default {
    fn factor(&self) -> f32;
}

impl Factor for f32 {
    fn factor(&self) -> Self {
        *self
    }
}

impl Factor for Voltage {
    fn factor(&self) -> f32 {
        self.0 + 1.0
    }
}
