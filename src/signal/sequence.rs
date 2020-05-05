use crate::*;
use crate::signal::clock::*;

pub struct Sequence<R: Rate = Hz44100> {
    clock: Clock<R>,
    position: usize,
    elements: Vec<Frequency>,
}

impl<R: Rate> Sequence<R> {
    pub fn new(clock: Clock<R>, elements: Vec<Frequency>) -> Self {
        Self {
            clock,
            position: 0,
            elements,
        }
    }
}

impl Signal for Sequence {
    type Sample = Frequency;

    fn sample(&mut self) -> Self::Sample {
        if self.clock.sample() {
            self.position += 1;
            self.position %= self.elements.len();
        }
        self.elements[self.position]
    }
}
