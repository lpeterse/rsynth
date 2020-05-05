use crate::*;
use std::f64::consts::PI;

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

    #[inline(never)]
    fn sample(&mut self) -> Voltage {
        // 1234: 100000000101110000000000000000000000000000000000000000000000000
        self.phi.0 = self.phi.0.wrapping_add(self.f.sample().delta());
        let x = self.phi.0 as f64;
        let x = x / 2f64.powi(31);
        let x = x * PI;
        let x = x.sin();
        let x = x * 2_147_483_646f64;
        let x = x + 2_147_483_648f64;
        let y = x.to_bits();
        let e = ((y >> 52) & 0x7ff) as u64;
        let e = 52 - (e - 1024);
        let m = ((y & 0xfffffffffffff) | 0x10000000000000) >> e;
        Voltage(m as u32)
    }
}

#[inline(never)]
fn a(x: f64) {
    println!("{:?}", x);
}

#[inline(never)]
fn b(x: u64) {
    println!("{} {:b}", x, x);
}