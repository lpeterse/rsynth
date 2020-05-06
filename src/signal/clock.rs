use crate::*;

#[derive(Debug)]
pub struct Clock<R: Rate = Hz44100> {
    f: Frequency<R>,
    phi: i32,
}

impl<R: Rate> Clock<R> {
    pub fn new(f: Frequency<R>) -> Self {
        Self { f, phi: 0 }
    }
}

impl<R: Rate> Signal for Clock<R> {
    type Sample = bool;

    fn sample(&mut self) -> Self::Sample {
        let delta = self.f.delta();
        self.phi = self.phi.wrapping_add(delta);
        self.phi < delta
    }
}
