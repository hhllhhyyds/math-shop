#[macro_export]
macro_rules! impl_arithmetic_trait {
    ($fl_ty: ty, $trait_0: tt, $trait_1: tt, $fn_0: tt, $fn_1: tt) => {
        impl core::ops::$trait_0 for $fl_ty {
            type Output = Self;

            #[inline]
            fn $fn_0(self, rhs: $fl_ty) -> Self {
                Self(self.0.$fn_0(rhs.0))
            }
        }

        impl core::ops::$trait_1 for $fl_ty {
            #[inline]
            fn $fn_1(&mut self, rhs: $fl_ty) {
                self.0.$fn_1(rhs.0)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_sum_product {
    ($fl_ty: ty) => {
        impl core::iter::Sum for $fl_ty {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                Self(iter.map(|x| x.0).sum())
            }
        }

        impl core::iter::Product for $fl_ty {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                Self(iter.map(|x| x.0).product())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_neg_trait {
    ($fl_ty: ty) => {
        impl core::ops::Neg for $fl_ty {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self::Output {
                Self(self.0.neg())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_debug_display_trait {
    ($fl_ty: ty) => {
        impl core::fmt::Debug for $fl_ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                core::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl core::fmt::Display for $fl_ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                core::fmt::Display::fmt(&self.0, f)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_complex_fns {
    ($fl_ty: ty, $inner_ty: ty, $fn: tt, $std_fn: tt) => {
        impl $fl_ty {
            #[inline]
            pub fn $fn(self) -> Self {
                #[cfg(feature = "std")]
                return Self(self.0.$std_fn());
                #[cfg(all(not(feature = "std"), feature = "libm"))]
                return Self(libm::Libm::<$inner_ty>::$fn(self.0));
            }
        }
    };
}

#[macro_export]
macro_rules! impl_float_const {
    ($fl_ty: tt,$name:tt, $value: expr) => {
        impl $fl_ty {
            pub const $name: $fl_ty = $fl_ty($value);
        }
    };
}

macro_rules! impl_approx_traits {
    ($fl_ty: ty, $inner_ty: ty) => {
        #[cfg(feature = "approx")]
        mod impl_approx {
            impl approx::AbsDiffEq for $fl_ty {
                type Epsilon = Self;

                fn default_epsilon() -> Self::Epsilon {
                    Self(<$inner_ty as approx::AbsDiffEq>::default_epsilon())
                }

                fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                    approx::AbsDiffEq::abs_diff_eq(&self.0, &other.0, epsilon.0)
                }
            }

            impl approx::RelativeEq for $fl_ty {
                fn default_max_relative() -> Self::Epsilon {
                    Self(<$inner_ty as approx::RelativeEq>::default_max_relative())
                }

                fn relative_eq(
                    &self,
                    other: &Self,
                    epsilon: Self::Epsilon,
                    max_relative: Self::Epsilon,
                ) -> bool {
                    approx::RelativeEq::relative_eq(&self.0, &other.0, epsilon.0, max_relative.0)
                }
            }

            impl approx::UlpsEq for $fl_ty {
                fn default_max_ulps() -> u32 {
                    <$inner_ty as approx::UlpsEq>::default_max_ulps()
                }

                fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                    approx::UlpsEq::ulps_eq(&self.0, &other.0, epsilon.0, max_ulps)
                }
            }
        }
    };
}

pub(crate) use impl_approx_traits;
pub(crate) use impl_arithmetic_trait;
pub(crate) use impl_complex_fns;
pub(crate) use impl_debug_display_trait;
pub(crate) use impl_float_const;
pub(crate) use impl_neg_trait;
pub(crate) use impl_sum_product;
