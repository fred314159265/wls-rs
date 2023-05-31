use wls::models::wls::Wls;

fn main() {
    let x_points = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let y_points = vec![1.0, 3.0, 4.0, 5.0, 2.0, 3.0, 4.0];
    let weights = vec![10.0, 1.0, 3.0, 8.0, 14.0, 21.0, 13.0];

    let wls = Wls::new(x_points, y_points, Some(weights));
    let point = wls.fit_linear_regression().unwrap();

    println!("Slope: {:?}", point.get_slope());
    println!("Intercept: {:?}", point.get_intercept());
}
