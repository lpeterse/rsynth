use crate::*;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Angle<R: Rate = Hz44100>(pub i32, PhantomData<*const R>);

impl<R: Rate> Angle<R> {
    pub const NULL: Self = Self(0, PhantomData);
    pub const PI: Self = Self(i32::MAX / 2, PhantomData);
}

/*impl<R: Rate> Angle<R> {
    pub fn add(self, f: Frequency<R>) -> Self {
        Self(self.0.wrapping_add(f.delta()), PhantomData)
    }
}*/

impl<R: Rate> Default for Angle<R> {
    fn default() -> Self {
        Self::NULL
    }
}
