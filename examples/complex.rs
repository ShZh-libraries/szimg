use std::ops::{Add, Sub, Mul, Div};

#[derive(PartialEq, Clone, Copy)]
pub struct Complex{
    pub real: f64,
    pub imaginary: f64,
}

impl Add for Complex {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            real: self.real * rhs.real - self.imaginary * rhs.imaginary,
            imaginary: self.real * rhs.imaginary + self.imaginary * rhs.real,
        }
    }
}

impl Div for Complex {
    type Output = Self;
    
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            real: self.real / rhs.real,
            imaginary: self.imaginary / rhs.imaginary,
        }
    }
}

impl Complex {
    pub fn conjugate(&self) -> Self {
        Self {
            real: self.real,
            imaginary: -self.imaginary,
        }
    }

    pub fn get_length(&self) -> f64 {
        self.real * self.real + self.imaginary * self.imaginary
    }
}

// Only to make cargo happy
fn main() {}