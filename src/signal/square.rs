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

    // mov     eax, dword, ptr, [rdi, +, 4]
    // add     eax, dword, ptr, [rdi]
    // mov     dword, ptr, [rdi], eax
    // shr     eax, 31
    // add     eax, 2147483647
    // ret
    #[inline(never)]
    fn sample(&mut self) -> Voltage {
        self.phi.0 = self.phi.0.wrapping_add(self.f.sample().0);
        if self.phi.0 >= 0 {
            Voltage(1.0)
        } else {
            Voltage(-1.0)
        }
    }
}
