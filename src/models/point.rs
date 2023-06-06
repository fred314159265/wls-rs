/// The calculated intercept and slope from the performed linear regression.
#[derive(Debug)]
pub struct Point {
    /// Intercept/offset of the fitted line.
    intercept: f64,
    /// Slope/gain of the fitted line.
    slope: f64,
}

impl Point {
    // Create a new linear fit.
    pub fn new(intercept: f64, slope: f64) -> Self {
        Self { intercept, slope }
    }

    /// Get the intercept/offset of this fitted line.
    pub fn get_intercept(&self) -> f64 {
        self.intercept
    }

    /// Get the slope/gain of this fitted line.
    pub fn get_slope(&self) -> f64 {
        self.slope
    }
}
