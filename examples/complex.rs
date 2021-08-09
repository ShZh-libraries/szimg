use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub imaginary: f64,
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            real: self.real * rhs.real - self.imaginary * rhs.imaginary,
            imaginary: self.real * rhs.imaginary + self.imaginary * rhs.real,
        }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
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

fn main() {}