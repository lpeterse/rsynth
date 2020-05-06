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
            phi: Angle::NULL,
            f,
        }
    }
}

impl<F: Signal<R, Sample = Frequency<R>>, R: Rate> Signal<R> for Saw<F, R> {
    type Sample = Voltage;

    #[inline(never)]
    fn sample(&mut self) -> Voltage {
        let x = self.phi.0.wrapping_add(self.f.sample().0);
        self.phi.0 = x;
        Voltage(x as f32)
    }
}
