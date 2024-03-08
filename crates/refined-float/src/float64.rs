use crate::macros::{
    impl_approx_traits, impl_arithmetic_trait, impl_complex_fns, impl_debug_display_trait,
    impl_float_const, impl_neg_trait, impl_sum_product,
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Float64(pub core::primitive::f64);

impl_arithmetic_trait!(Float64, Add, AddAssign, add, add_assign);
impl_arithmetic_trait!(Float64, Sub, SubAssign, sub, sub_assign);
impl_arithmetic_trait!(Float64, Mul, MulAssign, mul, mul_assign);
impl_arithmetic_trait!(Float64, Div, DivAssign, div, div_assign);
impl_arithmetic_trait!(Float64, Rem, RemAssign, rem, rem_assign);
impl_sum_product!(Float64);
impl_neg_trait!(Float64);
impl_debug_display_trait!(Float64);

impl Float64 {
    #[inline]
    pub fn abs(self) -> Self {
        #[cfg(feature = "std")]
        return Self(self.0.abs());
        #[cfg(all(not(feature = "std"), feature = "libm"))]
        return Self(libm::Libm::<f64>::fabs(self.0));
    }
}

impl_complex_fns!(Float64, core::primitive::f64, sqrt, sqrt);
impl_complex_fns!(Float64, core::primitive::f64, cos, cos);
impl_complex_fns!(Float64, core::primitive::f64, acos, acos);
impl_complex_fns!(Float64, core::primitive::f64, sin, sin);
impl_complex_fns!(Float64, core::primitive::f64, asin, asin);
impl_complex_fns!(Float64, core::primitive::f64, tan, tan);
impl_complex_fns!(Float64, core::primitive::f64, atan, atan);

impl_float_const!(Float64, ZERO, 0.0);
impl_float_const!(Float64, ONE, 1.0);

impl_approx_traits!(crate::Float64, f64);
