use crate::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Amp<S, F, V = Voltage, R = Hz44100>
where
    R: Rate,
    S: Signal<R, Sample = Voltage>,
    F: Signal<R, Sample = V>,
    V: Factor,
{
    rate: PhantomData<*const R>,
    signal: S,
    factor: F,
}

impl<S, F, V, R> Amp<S, F, V, R>
where
    R: Rate,
    S: Signal<R, Sample = Voltage>,
    F: Signal<R, Sample = V>,
    V: Factor,
{
    pub fn new(signal: S, factor: F) -> Self {
        Self {
            rate: PhantomData,
            signal,
            factor,
        }
    }
}

impl<S, F, V, R> Signal<R> for Amp<S, F, V, R>
where
    R: Rate,
    S: Signal<R, Sample = Voltage>,
    F: Signal<R, Sample = V>,
    V: Factor,
{
    type Sample = Voltage;

    fn sample(&mut self) -> Voltage {
        let x = self.signal.sample().0;
        let f = self.factor.sample().factor();
        Voltage(x * f)
    }
}
