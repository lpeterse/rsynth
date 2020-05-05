use crate::*;
use std::f32::consts::PI;

#[derive(Debug)]
pub struct Sine<F: Signal<R, Sample = Frequency<R>> = Frequency, R: Rate = Hz44100> {
    phi: Angle<R>,
    f: F,
}

impl<F: Signal<R, Sample = Frequency<R>>, R: Rate> Sine<F, R> {
    pub fn new(f: F) -> Self {
        Self {
            phi: Angle::NULL,
            f,
        }
    }
}

impl<F: Signal<R, Sample = Frequency<R>>, R: Rate> Signal<R> for Sine<F, R> {
    type Sample = Voltage;

    fn sample(&mut self) -> Voltage {
        self.phi = self.phi.add(self.f.sample());
        let rad = (self.phi.0 as f32) / 2.0f32.powi(32) * 2.0 * PI;
        let sin = rad.sin();
        let sin = sin + 1.0;
        let sin = sin * 2.0f32.powi(30);
        (sin as u32).into()
    }
}
