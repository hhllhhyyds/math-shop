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

pub(crate) use impl_arithmetic_trait;
pub(crate) use impl_complex_fns;
pub(crate) use impl_debug_display_trait;
pub(crate) use impl_neg_trait;
