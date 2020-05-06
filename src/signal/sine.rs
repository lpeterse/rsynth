use crate::*;

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

    // push    rax
    // mov     eax, dword, ptr, [rdi, +, 4]
    // add     eax, dword, ptr, [rdi]
    // mov     dword, ptr, [rdi], eax
    // cvtsi2sd xmm0, eax
    // mulsd   xmm0, qword, ptr, [rip, +, .LCPI3_0]
    // mulsd   xmm0, qword, ptr, [rip, +, .LCPI3_1]
    // call    qword, ptr, [rip, +, sin@GOTPCREL]
    // mulsd   xmm0, qword, ptr, [rip, +, .LCPI3_2]
    // cvttsd2si eax, xmm0
    // pop     rcx
    // ret
    #[inline(never)]
    fn sample(&mut self) -> Voltage {
        use std::f64::consts::PI;
        self.phi.0 = self.phi.0.wrapping_add(self.f.sample().delta());
        let x = self.phi.0 as f64;
        let x = x * 2f64.powi(-31);
        let x = x * PI;
        let x = x.sin();
        //let x = x * 2f64.powi(31);
        Voltage(x as f32)
    }
}
