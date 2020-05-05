use crate::*;

#[derive(Debug)]
pub struct Saw<F, R = Hz44100>
where
    R: Rate,
    F: Signal<R, Sample = Frequency<R>>,
{
    phi: Angle<R>,
    f: F,
}

impl<F: Signal<R, Sample = Frequency<R>>, R: Rate> Saw<F, R> {
    pub fn new(f: F) -> Self {
        Self {
            phi: Angle::default(),
            f,
        }
    }
}

impl<F: Signal<R, Sample = Frequency<R>>, R: Rate> Signal<R> for Saw<F, R> {
    type Sample = Voltage;

    fn sample(&mut self) -> Voltage {
        self.phi = self.phi.add(self.f.sample());
        self.phi.0.into()
    }
}
