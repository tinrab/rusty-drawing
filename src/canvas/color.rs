use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Color(u8, u8, u8);

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color(r, g, b)
    }

    // Credit: https://gist.github.com/mjackson/5311256
    pub fn from_hsl(hue: f64, saturation: f64, lightness: f64) -> Self {
        assert!(hue >= 0.0 && hue <= 1.0);
        assert!(saturation >= 0.0 && saturation <= 1.0);
        assert!(lightness >= 0.0 && lightness <= 1.0);
        let mut red = lightness;
        let mut green = lightness;
        let mut blue = lightness;
        if saturation > 0.0 {
            let hue_to_rgb = |p, q, t| {
                let t = if t < 0.0 {
                    t + 1.0
                } else if t > 1.0 {
                    t - 1.0
                } else {
                    t
                };
                if t < 1.0 / 6.0 {
                    p + (q - p) * 6.0 * t
                } else if t < 0.5 {
                    q
                } else if t < 2.0 / 3.0 {
                    p + (q - p) * (2.0 / 3.0 - t) * 6.0
                } else {
                    p
                }
            };
            let q = if lightness < 0.5 {
                lightness * (1.0 + saturation)
            } else {
                lightness + saturation - lightness * saturation
            };
            let p = 2.0 * lightness - q;
            red = hue_to_rgb(p, q, hue + 1.0 / 3.0);
            green = hue_to_rgb(p, q, hue);
            blue = hue_to_rgb(p, q, hue - 1.0 / 3.0);
        }
        Color(
            (red * 255.0).round() as u8,
            (green * 255.0).round() as u8,
            (blue * 255.0).round() as u8,
        )
    }

    pub fn random() -> Self {
        Self::from_hsl(rand::thread_rng().gen(), 0.85, 0.75)
    }

    pub fn red(&self) -> u8 {
        self.0
    }

    pub fn green(&self) -> u8 {
        self.1
    }

    pub fn blue(&self) -> u8 {
        self.2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hsl() {
        let c = Color::from_hsl(0.5, 0.5, 0.5);
        assert_eq!(c, Color::new(64, 191, 191));
        let c = Color::from_hsl(0.1, 0.8, 0.6);
        assert_eq!(c, Color::new(235, 169, 71));
    }
}
