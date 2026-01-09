/// RGB color representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }
}

/// RGBW color representation (with white channel)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RgbwColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub w: u8,
}

impl RgbwColor {
    pub fn new(r: u8, g: u8, b: u8, w: u8) -> Self {
        Self { r, g, b, w }
    }

    pub fn from_rgb(rgb: RgbColor) -> Self {
        Self {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            w: 0,
        }
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_from_hex() {
        let color = RgbColor::from_hex(0xFF00AA);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 170);
    }

    #[test]
    fn test_rgb_to_hex() {
        let color = RgbColor::new(255, 0, 170);
        assert_eq!(color.to_hex(), 0xFF00AA);
    }
}
