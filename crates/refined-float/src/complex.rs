use core::fmt::{Debug, Display};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[cfg(feature = "approx")]
use approx::AbsDiffEq;

use crate::{Float32, Float64};

pub trait FloatTraitsForComplex:
    Sized
    + Copy
    + Clone
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Neg<Output = Self>
    + Sum
    + Product
{
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn atan(&self) -> Self;
    fn sqrt(&self) -> Self;
    fn from_f64(x: f64) -> Self;
    fn from_f32(x: f32) -> Self;

    const ZERO: Self;
    const ONE: Self;

    #[inline]
    fn sin_cos(&self) -> (Self, Self) {
        (self.sin(), self.cos())
    }
    #[inline]
    fn square(&self) -> Self {
        self.mul(*self)
    }
}

impl FloatTraitsForComplex for Float32 {
    #[inline]
    fn cos(&self) -> Self {
        Float32::cos(*self)
    }

    #[inline]
    fn sin(&self) -> Self {
        Float32::sin(*self)
    }

    #[inline]
    fn sqrt(&self) -> Self {
        Float32::sqrt(*self)
    }

    #[inline]
    fn atan(&self) -> Self {
        Float32::atan(*self)
    }

    #[inline]
    fn from_f32(x: f32) -> Self {
        Self(x)
    }

    #[inline]
    fn from_f64(x: f64) -> Self {
        Self(x as f32)
    }

    const ZERO: Self = Float32::ZERO;
    const ONE: Self = Float32::ONE;
}

impl FloatTraitsForComplex for Float64 {
    #[inline]
    fn cos(&self) -> Self {
        Float64::cos(*self)
    }

    #[inline]
    fn sin(&self) -> Self {
        Float64::sin(*self)
    }

    #[inline]
    fn sqrt(&self) -> Self {
        Float64::sqrt(*self)
    }

    #[inline]
    fn atan(&self) -> Self {
        Float64::atan(*self)
    }

    #[inline]
    fn from_f32(x: f32) -> Self {
        Self(x as f64)
    }

    #[inline]
    fn from_f64(x: f64) -> Self {
        Self(x)
    }

    const ZERO: Self = Float64::ZERO;
    const ONE: Self = Float64::ONE;
}

#[derive(Clone, Copy, PartialEq)]
pub struct Complex<F: FloatTraitsForComplex> {
    pub real: F,
    pub imag: F,
}

impl<F: FloatTraitsForComplex> Complex<F> {
    #[inline]
    pub fn new(real: F, imag: F) -> Self {
        Self { imag, real }
    }

    #[inline]
    pub fn from_len_angle(len: F, angle: F) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(len * sin, len * cos)
    }

    #[inline]
    pub fn length(&self) -> F {
        (self.imag.square() + self.real.square()).sqrt()
    }

    #[inline]
    pub fn angle(&self) -> F {
        (self.imag / self.real).atan()
    }

    #[inline]
    pub fn to_len_angle(&self) -> (F, F) {
        let angle = (self.imag / self.real).atan();
        let len = (self.imag.square() + self.real.square()).sqrt();
        (len, angle)
    }

    #[inline]
    pub fn conjugate(&self) -> Self {
        Self::new(self.real, -self.imag)
    }

    #[inline]
    pub fn scale(&self, s: F) -> Self {
        Self::new(self.real * s, self.imag * s)
    }
}

impl<F: FloatTraitsForComplex> From<(F, F)> for Complex<F> {
    #[inline]
    fn from(value: (F, F)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<F: FloatTraitsForComplex> Add for Complex<F> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.real + rhs.real, self.imag + rhs.imag)
    }
}

impl<F: FloatTraitsForComplex> AddAssign for Complex<F> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.real.add_assign(rhs.real);
        self.imag.add_assign(rhs.imag);
    }
}

impl<F: FloatTraitsForComplex> Sub for Complex<F> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.real - rhs.real, self.imag - rhs.imag)
    }
}

impl<F: FloatTraitsForComplex> SubAssign for Complex<F> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.real.sub_assign(rhs.real);
        self.imag.sub_assign(rhs.imag);
    }
}

impl<F: FloatTraitsForComplex> Mul for Complex<F> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.real * rhs.real - self.imag * rhs.imag,
            self.real * rhs.imag + self.imag * rhs.real,
        )
    }
}

impl<F: FloatTraitsForComplex> MulAssign for Complex<F> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

impl<F: FloatTraitsForComplex> Div for Complex<F> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let x = rhs.real.square() + rhs.imag.square();
        Self::new(
            (self.real * rhs.real + self.imag * rhs.imag) / x,
            (self.imag * rhs.real - self.real * rhs.imag) / x,
        )
    }
}

impl<F: FloatTraitsForComplex> DivAssign for Complex<F> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs);
    }
}

impl<F: FloatTraitsForComplex> Neg for Complex<F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.real, -self.imag)
    }
}

impl<F: FloatTraitsForComplex> Sum for Complex<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(F::ZERO, F::ZERO), |a, b| a + b)
    }
}

impl<F: FloatTraitsForComplex> core::iter::Product for Complex<F> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(F::ONE, F::ZERO), |a, b| a * b)
    }
}

impl<F: FloatTraitsForComplex + Debug + PartialOrd> Debug for Complex<F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}({:#?} {} {:#?} I)",
            stringify!(Complex<F>),
            self.real,
            if self.imag > F::ZERO { '+' } else { '-' },
            if self.imag > F::ZERO {
                self.imag
            } else {
                -self.imag
            },
        )
    }
}

impl<F: FloatTraitsForComplex + Display + PartialOrd> Display for Complex<F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} {} {} I",
            self.real,
            if self.imag > F::ZERO { '+' } else { '-' },
            if self.imag > F::ZERO {
                self.imag
            } else {
                -self.imag
            },
        )
    }
}

#[cfg(feature = "approx")]
impl<F: FloatTraitsForComplex + AbsDiffEq<Epsilon = F> + PartialEq> approx::AbsDiffEq
    for Complex<F>
{
    type Epsilon = F;

    fn default_epsilon() -> Self::Epsilon {
        F::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.real.abs_diff_eq(&other.real, epsilon) && self.imag.abs_diff_eq(&other.imag, epsilon)
    }
}
