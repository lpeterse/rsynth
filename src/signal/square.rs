use crate::*;

#[derive(Debug)]
pub struct Square<F, R = Hz44100>
where
    R: Rate,
    F: Signal<R, Sample = Frequency<R>>,
{
    phi: Angle<R>,
    f: F,
}

impl<F: Signal<R, Sample = Frequency<R>>, R: Rate> Square<F, R> {
    pub fn new(f: F) -> Self {
        Self {
            phi: Angle::default(),
            f,
        }
    }
}

impl<F: Signal<R, Sample = Frequency<R>>, R: Rate> Signal<R> for Square<F, R> {
    type Sample = Voltage;

    fn sample(&mut self) -> Voltage {
        self.phi = self.phi.add(self.f.sample());
        if self.phi.0 > u32::MAX / 2 {
            Voltage::MAX
        } else {
            Voltage::MIN
        }
    }
}
