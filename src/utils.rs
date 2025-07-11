/// Generates a random `f64` in the range [0, 1) using fastrand
#[inline]
pub fn random_double() -> f64 {
    fastrand::f64()
}

/// Generates a random `f64` in the specified [min, max) range
///
/// # Panics
/// Panics if `max < min` to prevent undefined behavior
#[inline]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    assert!(max >= min, "max must be greater than or equal to min");
    min + (max - min) * random_double()
}

/// Converts degrees to radians
///
/// # Examples
/// ```
/// assert_eq!(degrees_to_radians(180.0), std::f64::consts::PI);
/// ```
#[inline]
pub const fn degrees_to_radians(degrees: f64) -> f64 {
    degrees.to_radians()
}

/// Converts radians to degrees
///
/// # Examples
/// ```
/// assert_eq!(radians_to_degrees(std::f64::consts::PI), 180.0);
/// ```
#[inline]
pub const fn radians_to_degrees(radians: f64) -> f64 {
    radians.to_degrees()
}
