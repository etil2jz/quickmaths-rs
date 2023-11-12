/* Trigonometry functions */

/// Approximation of the sine of an angle using Taylor sequences.
/// For applications that do not require absolute precision.
pub fn fm_sin(angle: f64) -> f64 {
    return angle - angle.powi(3) / 6.0 + angle.powi(5) / 120.0 - angle.powi(7) / 5040.0;
}

/// Approximation of the cosine of an angle using Taylor sequences.
/// For applications that do not require absolute precision.
pub fn fm_cos(angle: f64) -> f64 {
    return 1.0 - angle.powi(2) / 2.0 + angle.powi(4) / 24.0 - angle.powi(6) / 720.0;
}
