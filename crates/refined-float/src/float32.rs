use crate::macros::{
    impl_approx_traits, impl_arithmetic_trait, impl_complex_fns, impl_debug_display_trait,
    impl_float_const, impl_neg_trait, impl_sum_product,
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Float32(pub core::primitive::f32);

impl_arithmetic_trait!(Float32, Add, AddAssign, add, add_assign);
impl_arithmetic_trait!(Float32, Sub, SubAssign, sub, sub_assign);
impl_arithmetic_trait!(Float32, Mul, MulAssign, mul, mul_assign);
impl_arithmetic_trait!(Float32, Div, DivAssign, div, div_assign);
impl_arithmetic_trait!(Float32, Rem, RemAssign, rem, rem_assign);
impl_sum_product!(Float32);
impl_neg_trait!(Float32);
impl_debug_display_trait!(Float32);

impl Float32 {
    #[inline]
    pub fn abs(self) -> Self {
        #[cfg(feature = "std")]
        return Self(self.0.abs());
        #[cfg(all(not(feature = "std"), feature = "libm"))]
        return Self(libm::Libm::<f32>::fabs(self.0));
    }
}

impl_complex_fns!(Float32, core::primitive::f32, sqrt, sqrt);
impl_complex_fns!(Float32, core::primitive::f32, cos, cos);
impl_complex_fns!(Float32, core::primitive::f32, acos, acos);
impl_complex_fns!(Float32, core::primitive::f32, sin, sin);
impl_complex_fns!(Float32, core::primitive::f32, asin, asin);
impl_complex_fns!(Float32, core::primitive::f32, tan, tan);
impl_complex_fns!(Float32, core::primitive::f32, atan, atan);

impl_float_const!(Float32, ZERO, 0.0);
impl_float_const!(Float32, ONE, 1.0);

impl_approx_traits!(crate::Float32, f32);
