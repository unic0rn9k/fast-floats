//! # Fast Floats
//! This project is forked from [bluss/fast-floats](https://github.com/bluss/fast-floats).
//!
//! **Changes from original project:**
//! - Fast floats implement deref to their source float type
//!
//! # Original docs
//! [Docs for `Fast` struct ](https://docs.rs/fast-floats/latest/fast_floats/struct.Fast.html)
//!
//! Experimental (unstable) “fast-math” wrappers for f32, f64
//!
//! These wrappers enable the [“fast-math”][1] flags for the operations
//! where there are intrinsics for this (add, sub, mul, div, rem).
//! The wrappers exist so that we have a quick & easy way **to experiment**
//! with fast math flags and further that feature in Rust.
//!
//! Note that as of this writing, the Rust instrinsics use the “fast” flag
//! documented in the langref; this enables all the float flags.
//!
//! [1]: https://llvm.org/docs/LangRef.html#fast-math-flags
//!
//! # Rust Version
//!
//! This crate is nightly only and experimental. Breaking changes can occur at
//! any time, if changes in Rust require it.
#![no_std]
#![feature(core_intrinsics, const_trait_impl)]

extern crate core as std;

use std::intrinsics::{fadd_fast, fdiv_fast, fmul_fast, frem_fast, fsub_fast};
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};

/// “fast-math” wrapper for f32 and f64.
///
/// The `Fast` type enforces no invariant and can hold any f32, f64 values.
/// See crate docs for more details.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Fast<F>(F);

impl<F> const Deref for Fast<F> {
    type Target = F;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F> DerefMut for Fast<F> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// This is actually a bad idea, but is required for my use cases.
/// Creating a Fast float should be unsafe - as fast floats use `core_intrinsics`.
impl<F> From<F> for Fast<F> {
    #[inline(always)]
    fn from(f: F) -> Self {
        Self(f)
    }
}

/// “fast-math” wrapper for `f64`
pub type FF64 = Fast<f64>;
/// “fast-math” wrapper for `f32`
pub type FF32 = Fast<f32>;

impl<F> Fast<F> {
    /// Create a new fast value
    ///
    /// # Safety
    ///
    /// The value can be used with the `fast_fadd` etc intrinsics without checks after it has been
    /// created like this. Refer to Rust and other sources for documentation on which operations
    /// are valid (might change with time).
    ///
    /// Be wary of operations creating invalid values in `Fast` which they could potentially do
    /// depending on the operation.
    pub const unsafe fn new(value: F) -> Self {
        Fast(value)
    }
}

macro_rules! impl_op {
    ($($name:ident, $method:ident, $intrins:ident;)*) => {
        $(
        // Fast<F> + F
        impl $name<f64> for Fast<f64> {
            type Output = Self;
            #[inline(always)]
            fn $method(self, rhs: f64) -> Self::Output {
                unsafe {
                    Fast($intrins(self.0, rhs))
                }
            }
        }

        impl $name<f32> for Fast<f32> {
            type Output = Self;
            #[inline(always)]
            fn $method(self, rhs: f32) -> Self::Output {
                unsafe {
                    Fast($intrins(self.0, rhs))
                }
            }
        }

        // F + Fast<F>
        impl $name<Fast<f64>> for f64 {
            type Output = Fast<f64>;
            #[inline(always)]
            fn $method(self, rhs: Fast<f64>) -> Self::Output {
                Fast(self).$method(rhs.0)
            }
        }

        impl $name<Fast<f32>> for f32 {
            type Output = Fast<f32>;
            #[inline(always)]
            fn $method(self, rhs: Fast<f32>) -> Self::Output {
                Fast(self).$method(rhs.0)
            }
        }

        // Fast<F> + Fast<F>
        impl $name for Fast<f64> {
            type Output = Self;
            #[inline(always)]
            fn $method(self, rhs: Self) -> Self::Output {
                self.$method(rhs.0)
            }
        }

        impl $name for Fast<f32> {
            type Output = Self;
            #[inline(always)]
            fn $method(self, rhs: Self) -> Self::Output {
                self.$method(rhs.0)
            }
        }
        )*

    }
}

macro_rules! impl_assignop {
    ($($name:ident, $method:ident, $optrt:ident, $opmth:ident;)*) => {
        $(
        impl<F, Rhs> $name<Rhs> for Fast<F>
            where Self: $optrt<Rhs, Output=Self> + Copy,
        {
            #[inline(always)]
            fn $method(&mut self, rhs: Rhs) {
                *self = (*self).$opmth(rhs)
            }
        }
        )*

    }
}

impl_op! {
    Add, add, fadd_fast;
    Sub, sub, fsub_fast;
    Mul, mul, fmul_fast;
    Div, div, fdiv_fast;
    Rem, rem, frem_fast;
}

impl_assignop! {
    AddAssign, add_assign, Add, add;
    SubAssign, sub_assign, Sub, sub;
    MulAssign, mul_assign, Mul, mul;
    DivAssign, div_assign, Div, div;
    RemAssign, rem_assign, Rem, rem;
}

use std::fmt;
macro_rules! impl_format {
    ($($name:ident)+) => {
        $(
        impl<F: fmt::$name> fmt::$name for Fast<F> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        )+
    }
}

impl_format!(Debug Display LowerExp UpperExp);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_op {
        ($($op:tt)+) => {
            $(
                assert_eq!(Fast(2.) $op Fast(1.), Fast(2. $op 1.));
            )+
        }
    }

    #[test]
    fn each_op() {
        test_op!(+ - * / %);
    }

    macro_rules! assign_op {
        ($($x:literal $op:tt $y:literal is $z:literal ;)+) => {
            $(
                let mut x = Fast($x);
                x $op Fast($y);
                assert_eq!(x, Fast($z));
            )+
        }
    }

    #[test]
    fn assign_ops() {
        assign_op!(
            1. += 2. is 3.;
            1. -= 2. is -1.;
            2. *= 2. is 4.;
            2. /= 2. is 1.;
            5. %= 2. is 1.;
        );
    }

    #[test]
    fn deref() {
        let a = unsafe { FF32::new(2.) };
        assert_eq!(a.sin(), 2f32.sin())
    }

    #[test]
    fn conversion() {
        let f = |_: FF32| {};
        f(0f32.into());

        let f = |_: f32| {};
        unsafe { f(*FF32::new(0.)) };
    }
}
