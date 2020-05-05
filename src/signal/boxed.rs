use crate::*;

pub struct BoxedSignal<T, R: Rate = Hz44100>(Box<dyn Signal<R, Sample = T>>);

impl<T, R: Rate> BoxedSignal<T, R>
where
    T: Copy + Clone + std::fmt::Debug + Default
{
    pub fn new<S: Signal<R, Sample = T> + 'static>(s: S) -> Self {
        Self(Box::new(s))
    }
}

impl<T, R: Rate> Signal<R> for BoxedSignal<T, R>
where
    T: Copy + Clone + std::fmt::Debug + Default,
{
    type Sample = T;

    fn sample(&mut self) -> Self::Sample {
        use std::borrow::BorrowMut;
        let self_: &mut dyn Signal<R, Sample = T> = self.0.borrow_mut();
        self_.sample()
    }
}
