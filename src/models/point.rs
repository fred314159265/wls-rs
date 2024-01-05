use num_traits::float::FloatCore;

/// The calculated intercept and slope from the performed linear regression.
#[derive(Debug)]
pub struct Point<T: FloatCore> {
    /// Intercept/offset of the fitted line.
    intercept: T,
    /// Slope/gain of the fitted line.
    slope: T,
}

impl<T: FloatCore> Point<T> {
    // Create a new linear fit.
    pub fn new(intercept: T, slope: T) -> Self {
        Self { intercept, slope }
    }

    /// Get the intercept/offset of this fitted line.
    pub fn get_intercept(&self) -> T {
        self.intercept
    }

    /// Get the slope/gain of this fitted line.
    pub fn get_slope(&self) -> T {
        self.slope
    }
}
