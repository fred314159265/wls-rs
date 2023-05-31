# wls-rust

Weighted linear regression in pure Rust w/o any 3d party dependencies or frameworks. 

Forked to expose as a library crate - the hard work was done by vspaz here: https://github.com/vspaz/wls-rs

### How-to

Use in your project by adding this line to your `Cargo.toml`:

```toml
wls = {git = "https://github.com/fred314159265/wls-rs.git"}
```

```rust
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
```

## Description

WLS is based on the OLS method and help solve problems of model inadequacy or violations of the basic regression
assumptions.

Estimating a linear regression with WLS is useful, but can appear to be daunting w/o special stats packages, such as
Python statsmodels or Pandas.

## References

- [Wikipedia: Weighted least squares](https://en.wikipedia.org/wiki/Weighted_least_squares)
- [Introduction to Linear Regression Analysis, 5th edition](https://tinyurl.com/y3clfnrs)
- [Least Squares Regression Analysis in Terms of Linear Algebra](https://tinyurl.com/y485qhlg) 
