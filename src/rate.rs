pub trait Rate: Copy + Clone + std::fmt::Debug + Sized + Default {
    const RECIPROCAL: i32;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Hz44100 {}

impl Rate for Hz44100 {
    const RECIPROCAL: i32 = (u32::MAX / 44_100) as i32;
}
