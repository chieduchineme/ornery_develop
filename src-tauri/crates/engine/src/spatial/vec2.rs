#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn magnitude(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[inline]
    pub fn dist(self, other: Vec2) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Unit vector from self toward other. Returns zero vector if coincident.
    #[inline]
    pub fn direction_to(self, other: Vec2) -> Vec2 {
        let d = self.dist(other).max(0.001);
        Vec2::new((other.x - self.x) / d, (other.y - self.y) / d)
    }

    #[inline]
    pub fn scale(self, s: f64) -> Vec2 {
        Vec2::new(self.x * s, self.y * s)
    }

    #[inline]
    pub fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }

    #[inline]
    pub fn clamp_to_pitch(self) -> Vec2 {
        Vec2::new(self.x.clamp(2.0, 98.0), self.y.clamp(3.0, 97.0))
    }
}
