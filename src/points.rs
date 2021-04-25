// I couldn't find a simple crate that does this and nothing else, so here we go.
// Yes, some of this is copied from rust docs, because they just happen to use
// Points as convenient examples.

use std::f64::consts;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Mul for Point {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign for Point {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}

impl Div for Point {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl DivAssign for Point {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: - self.x,
            y: - self.y,
        }
    }
}


impl Point {
    /// Initialize from cartesian coords
    pub fn xy(x: f64, y: f64) -> Self {
        Self { x: x, y: y }
    }

    /// Initialize from polar coords
    pub fn rth(r: f64, th: f64) -> Self {
        Self {
            x: r * th.cos(),
            y: r * th.sin()
        }
    }
    
    pub fn scale(mut self, c: f64) -> Self {
        self.x *= c;
        self.y *= c;
        self
    }

    pub fn modulo(&self, n: Point) -> Self {
        Point {
            x: modulo(self.x, n.x),
            y: modulo(self.y, n.y)
        }
    }

    pub fn r(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn th(&self) -> f64 {
        mod360(self.y.atan2(self.x))
    }
}

/// This is a better modulo function. (returns x mod n)
/// Normal mod:  -1 % n         ==> 1
/// This function: -1.modulo(n) ==> n - 1
pub fn modulo(x: f64, n: f64) -> f64 {
    n * ((x / n) - (x / n).floor())
}

/// Casts the angle between 0 and 360deg (in radians)
pub fn mod360(th: f64) -> f64 {
    modulo(th, 2.0 * consts::PI)
}

/// Casts the angle between -180deg and 180deg (in radians)
pub fn mod360_symmetric(th: f64) -> f64 {
    modulo(th + consts::PI, 2.0 * consts::PI) - consts::PI
}


/// Wrapper around Point with some extra values for relative positioning.
#[derive(Debug, Clone, PartialEq)]
pub enum RelativePoint {
    POINT(Point),
    CENTER
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Anchor {
    CENTER,
    NW,
    NE,
    SE,
    SW
}

impl Anchor {
    pub fn into_point(self, width: f64, height: f64) -> Point {
        match self {
            Anchor::NW => Point::xy(0.0, height),
            Anchor::NE => Point::xy(-width, height),
            Anchor::SE => Point::xy(width, 0.0),
            Anchor::SW => Point::xy(-width, 0.0),
            Anchor::CENTER => Point::xy(-width / 2.0, height / 2.0),
        }
    }
}

// Helpful angles
pub const ANGLE_EAST: f64 = 0.0;
pub const ANGLE_WEST: f64 = consts::PI;
pub const ANGLE_NORTH: f64 = consts::PI / 2.0;
pub const ANGLE_SOUTH: f64 = 3.0 * consts::PI / 2.0;