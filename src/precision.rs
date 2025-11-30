/* In Millington's description of his precision.h file, he states the reasoning for precision.h:
    Because Cyclone is designed to work at either single or double
 * precision, mathematical functions such as sqrt cannot be used
 * in the source code or headers. This file provides defines for
 * the real number type and mathematical formulae that work on it.
 */

//If we want double precision, we can switch this to 'f64' and adjust the constants accordingly.
pub type Real = f32;

// Maximum presentable Real value (like REAL_MAX in Cyclone).
pub const REAL_MAX: Real = Real::MAX;

// Small episloon value used for numerical comparisons (like real_epislon).
pub const REAL_EPSILON: Real = Real::EPSILON;

// Pi constant in engine precision
pub const REAL_PI: Real = std::f32::consts::PI;

// Square root in engine precision
#[inline]
pub fn real_sqrt(x:Real) -> Real {
    x.sqrt()
}

// Power in engine precision (like real_pow)
#[inline]
pub fn real_pow(base: Real, exponent: Real) -> Real {
    base.powf(exponent)
}

//Absolute value in engine precision (like real_abs).
#[inline]
pub fn real_abs(x:Real) -> Real {
    x.abs()
}