pub trait Rate: Copy + Clone + std::fmt::Debug + Sized + Default {
    const RECIPROCAL: u32;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Hz44100 {}

impl Rate for Hz44100 {
    const RECIPROCAL: u32 = u32::MAX / 44_100;
}
