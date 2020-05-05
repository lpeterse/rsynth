#[derive(Debug, Clone, Copy, Default)]
pub struct Voltage(pub u32);

impl Voltage {
    pub const MIN: Self = Self(u32::MIN);
    pub const MAX: Self = Self(u32::MAX);
}

impl From<u32> for Voltage {
    fn from(x: u32) -> Self {
        Self(x)
    }
}

impl Into<u32> for Voltage {
    fn into(self) -> u32 {
        self.0
    }
}

impl From<f32> for Voltage {
    fn from(x: f32) -> Self {
        if x >= 1.0 {
            Self::MAX
        } else if x <= 0.0 {
            Self::MIN
        } else {
            Self((x * (u32::MAX as f32)) as u32)
        }
    }
}

impl Into<f32> for Voltage {
    fn into(self) -> f32 {
        (self.0 as f32) / (u32::MAX as f32)
    }
}
