use crate::*;

use std::marker::PhantomData;

#[derive(Debug)]
pub struct Sum<S: Signal<R>, R: Rate = Hz44100> {
    rate: PhantomData<*const R>,
    signals: Vec<S>,
}

impl<S: Signal<R>, R: Rate> Sum<S, R> {
    pub fn new<Signals: Into<Vec<S>>>(signals: Signals) -> Self {
        Self {
            rate: PhantomData,
            signals: signals.into(),
        }
    }
}

impl<S, R> Signal<R> for Sum<S, R>
where
    S: Signal<R, Sample = Voltage>,
    R: Rate,
{
    type Sample = Voltage;

    fn sample(&mut self) -> Voltage {
        let mut sum: f32 = 0.0;
        for signal in &mut self.signals {
            sum += signal.sample().0;
        }
        Voltage(sum / self.signals.len() as f32)
    }
}

impl<T, R> From<Vec<BoxedSignal<T, R>>> for Sum<BoxedSignal<T, R>, R>
where
    R: Rate,
    T: Copy + Clone + std::fmt::Debug + Default,
{
    fn from(signals: Vec<BoxedSignal<T, R>>) -> Self {
        Self {
            rate: PhantomData,
            signals,
        }
    }
}

#[macro_export]
macro_rules! sum(
    () => ( Sum::new2(Vec::<BoxedSignal<Voltage>>::with_capacity(0)) );
    ( $e:expr $(, $more:expr)* ) => {{
        let mut v: Vec<BoxedSignal<Voltage>> = vec![];
        sum!(xxx v, $e $(, $more)*);
        Sum::from(v)
    }};

    ( xxx $v:expr ) => ();
    ( xxx $v:expr, $e:expr ) => ( $v.push(BoxedSignal::new($e)); );
    ( xxx $v:expr, $e:expr $(, $more:expr)* ) => {{
        $v.push(BoxedSignal::new($e));
        sum!(xxx $v $(, $more)*);
    }};
);
