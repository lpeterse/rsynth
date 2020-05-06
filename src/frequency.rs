use crate::*;
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, Default)]
pub struct Frequency<R: Rate = Hz44100>(pub i32, PhantomData<*const R>);

impl <R: Rate> Frequency<R> {
    pub fn delta(self) -> i32 {
        self.0
    }
}

pub const fn hz(f: f32) -> Frequency<Hz44100> {
    let x = f * Hz44100::RECIPROCAL as f32;
    Frequency(x as i32, PhantomData)
}

pub const C3: Frequency = hz(130.813);
pub const D3: Frequency = hz(146.832);
pub const E3: Frequency = hz(164.814);
pub const F3: Frequency = hz(174.614);
pub const G3: Frequency = hz(195.998);
pub const A3: Frequency = hz(220.000);
pub const H3: Frequency = hz(246.942);

pub const C4: Frequency = hz(261.626);
pub const D4: Frequency = hz(293.665);
pub const E4: Frequency = hz(329.628);
pub const F4: Frequency = hz(349.228);
pub const G4: Frequency = hz(391.995);
pub const A4: Frequency = hz(440.000);
pub const H4: Frequency = hz(493.883);

pub const C5: Frequency = hz(523.251);

impl Signal for Frequency {
    type Sample = Self;

    fn sample(&mut self) -> Self {
        Self(self.0, PhantomData)
    }
}
