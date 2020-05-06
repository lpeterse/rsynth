mod amp;
mod boxed;
mod clock;
mod factor;
mod multiple;
mod saw;
mod sequence;
mod sine;
mod square;
mod sum;

pub use self::amp::*;
pub use self::boxed::*;
pub use self::clock::*;
pub use self::factor::*;
pub use self::multiple::*;
pub use self::saw::*;
pub use self::sequence::*;
pub use self::sine::*;
pub use self::square::*;
pub use self::sum::*;

use crate::*;

pub trait Signal<S: Rate = Hz44100> {
    type Sample: Clone + Copy + std::fmt::Debug + Default;

    fn sample(&mut self) -> Self::Sample;
}

impl <R: Rate> Signal<R> for f32 {
    type Sample = Self;

    fn sample(&mut self) -> Self {
        *self
    }
}

pub fn clock(f: Frequency) -> Clock {
    Clock::new(f)
}

pub fn saw<F: Signal<Sample = Frequency>>(f: F) -> Saw<F> {
    Saw::new(f)
}

pub fn sine<F: Signal<Sample = Frequency>>(f: F) -> Sine<F> {
    Sine::new(f)
}


pub fn square<F: Signal<Sample = Frequency>>(f: F) -> Square<F> {
    Square::new(f)
}

pub fn sum<S, Signals>(signals: Signals) -> Sum<S>
where
    S: Signal<Sample = Voltage>,
    Signals: Into<Vec<S>>,
{
    Sum::new(signals)
}

pub fn multiple<S: Signal>(s: S) -> Multiple<S> {
    Multiple::new(s)
}
