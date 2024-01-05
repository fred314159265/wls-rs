use num_traits::float::FloatCore;
use num_traits::NumCast;

use crate::asserts::assert::{assert_have_same_size, assert_have_size_greater_than_two};
use crate::models::point::Point;

/// Weighted least squares linear regression fit calculator.
pub struct Wls<T: FloatCore> {
    x_points: Vec<T>,
    y_points: Vec<T>,
    weights: Vec<T>,
}

fn populate_weights<T: FloatCore>(capacity: &[T], value: T) -> Vec<T> {
    vec![value; capacity.len()]
}

impl<T: FloatCore> Wls<T> {
    /// Create a new instance of the weighted least square linear regression calculator.
    pub fn new(x_points: Vec<T>, y_points: Vec<T>, weights: Option<Vec<T>>) -> Self {
        let mut weights_normalized: Vec<_> = vec![];

        assert_have_same_size(&x_points, &y_points);
        if let Some(weights) = weights {
            weights_normalized = weights;
            assert_have_same_size(x_points.as_slice(), weights_normalized.as_slice());
        }
        assert_have_size_greater_than_two(x_points.as_slice());

        if weights_normalized.is_empty() {
            weights_normalized = populate_weights(&x_points, NumCast::from(1.0).unwrap());
        }
        Self {
            x_points,
            y_points,
            weights: weights_normalized,
        }
    }

    /// Perform the linear regression and return the fit, if calculable.
    pub fn fit_linear_regression(&self) -> Option<Point<T>> {
        let mut sum_of_weights: T = T::zero();
        let mut sum_of_products_of_weights_and_x_squared: T = T::zero();
        let mut sum_of_products_of_x_and_y_and_weights: T = T::zero();
        let mut sum_of_products_of_xi_and_wi: T = T::zero();
        let mut sum_of_products_of_y_and_weights: T = T::zero();

        let mut xi: T;
        let mut yi: T;
        let mut wi: T;
        let mut product_of_xi_and_wi: T;

        for i in 0..self.x_points.len() {
            xi = self.x_points[i];
            yi = self.y_points[i];
            wi = self.weights[i];

            sum_of_weights = sum_of_weights + wi;
            product_of_xi_and_wi = xi * wi;
            sum_of_products_of_xi_and_wi = sum_of_products_of_xi_and_wi + product_of_xi_and_wi;
            sum_of_products_of_x_and_y_and_weights =
                sum_of_products_of_x_and_y_and_weights + product_of_xi_and_wi * yi;
            sum_of_products_of_y_and_weights = sum_of_products_of_y_and_weights + yi * wi;
            sum_of_products_of_weights_and_x_squared =
                sum_of_products_of_weights_and_x_squared + product_of_xi_and_wi * xi;
        }

        let dividend = sum_of_weights * sum_of_products_of_x_and_y_and_weights
            - sum_of_products_of_xi_and_wi * sum_of_products_of_y_and_weights;
        let divisor = sum_of_weights * sum_of_products_of_weights_and_x_squared
            - sum_of_products_of_xi_and_wi * sum_of_products_of_xi_and_wi;
        if divisor == T::zero() {
            return None;
        }
        let slope = dividend / divisor;
        let intercept = (sum_of_products_of_y_and_weights - slope * sum_of_products_of_xi_and_wi)
            / sum_of_weights;

        Some(Point::new(intercept, slope))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asserts::assert::{assert_almost_equal, assert_true};
    use crate::models::point::Point;

    pub fn assert_model_can_be_fit<T: FloatCore>(wls: &Wls<T>) -> Point<T> {
        return match wls.fit_linear_regression() {
            Some(point) => point,
            None => panic!("can't fit linear regression"),
        };
    }

    #[test]
    fn test_wls_model_with_weights_ok() {
        let x_points: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let y_points: Vec<f32> = vec![1.0, 3.0, 4.0, 5.0, 2.0, 3.0, 4.0];
        let weights = vec![1.0, 2.0, 3.0, 1.0, 8.0, 1.0, 5.0];

        let wls = Wls::new(x_points, y_points, Some(weights));

        let point: Point<_> = assert_model_can_be_fit(&wls);
        assert_almost_equal(2.14285714, point.get_intercept(), 1.0e-6);
        assert_almost_equal(0.150862, point.get_slope(), 1.0e-6);
    }

    #[test]
    fn test_wls_model_with_stable_weights_ok() {
        let x_points: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let y_points: Vec<f64> = vec![1.0, 3.0, 4.0, 5.0, 2.0, 3.0, 4.0];

        let wls = Wls::new(x_points, y_points, None);

        let point: Point<_> = assert_model_can_be_fit(&wls);
        assert_almost_equal(2.14285714, point.get_intercept(), 1.0e-6);
        assert_eq!(0.25, point.get_slope());
    }

    #[test]
    fn test_horizontal_line_ok() {
        let x_points: Vec<f64> = vec![0.0, 1.0];
        let y_points: Vec<f64> = vec![10.0, 10.0];

        let wls = Wls::new(x_points, y_points, None);

        let point: Point<f64> = assert_model_can_be_fit(&wls);
        assert_eq!(10.0, point.get_intercept());
        assert_eq!(0.0, point.get_slope());
    }

    #[test]
    fn test_vertical_line_ok() {
        let x_points = vec![1.0, 1.0];
        let y_points = vec![0.0, 1.0];

        let wls = Wls::new(x_points, y_points, None);
        assert_true(wls.fit_linear_regression().is_none());
    }

    #[test]
    fn test_run_uphill_ok() {
        let x_points: Vec<f64> = vec![0.0, 1.0];
        let y_points: Vec<f64> = vec![0.0, 1.0];

        let wls = Wls::new(x_points, y_points, None);

        let point: Point<f64> = assert_model_can_be_fit(&wls);
        assert_eq!(0.0, point.get_intercept());
        assert_eq!(1.0, point.get_slope());
    }

    #[test]
    fn test_run_downhill_ok() {
        let x_points: Vec<f64> = vec![1.0, 0.0];
        let y_points: Vec<f64> = vec![0.0, 1.0];

        let wls = Wls::new(x_points, y_points, None);

        let point: Point<f64> = assert_model_can_be_fit(&wls);
        assert_eq!(1.0, point.get_intercept());
        assert_eq!(-1.0, point.get_slope());
    }
}
