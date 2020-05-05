use crate::*;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Multiple<S, R = Hz44100>
where
    S: Signal<R>,
    R: Rate,
{
    count: u64,
    inner: Rc<RefCell<Inner<S, R>>>,
}

impl<S, R> Clone for Multiple<S, R>
where
    S: Signal<R>,
    R: Rate,
{
    fn clone(&self) -> Self {
        Self {
            count: self.count,
            inner: self.inner.clone()
        }
    }
}

impl<S, R> Multiple<S, R>
where
    S: Signal<R>,
    R: Rate,
{
    pub fn new(s: S) -> Self {
        let inner = Inner {
            count: 0,
            signal: s,
            sample: <S as Signal<R>>::Sample::default(),
        };
        Self {
            count: 0,
            inner: Rc::new(RefCell::new(inner)),
        }
    }
}

impl<S, R> Signal<R> for Multiple<S, R>
where
    S: Signal<R>,
    R: Rate,
{
    type Sample = <S as Signal<R>>::Sample;

    fn sample(&mut self) -> Self::Sample {
        let mut inner = self.inner.borrow_mut();
        if self.count >= inner.count {
            inner.sample = inner.signal.sample();
            inner.count += 1;
        }
        self.count = inner.count;
        inner.sample
    }
}

#[derive(Debug, Clone)]
struct Inner<S, R = Hz44100>
where
    S: Signal<R>,
    R: Rate,
{
    count: u64,
    signal: S,
    sample: <S as Signal<R>>::Sample,
}
