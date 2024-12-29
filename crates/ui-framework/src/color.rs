use std::ops::Mul;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsla {
    pub hue: f32,        // Hue component (0-360 degrees)
    pub saturation: f32, // Saturation component (0-1)
    pub lightness: f32,  // Lightness component (0-1)
    pub alpha: f32,      // Alpha/opacity (0-1)
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Background {
    pub color: Hsla,
}

impl Mul<f32> for Hsla {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            hue: self.hue,
            saturation: self.saturation,
            lightness: self.lightness,
            alpha: self.alpha * rhs,
        }
    }
}
