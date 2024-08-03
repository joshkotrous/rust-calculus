pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn numerical_derivative<F>(f: F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (f(x + h) - f(x)) / h
}

fn main() {
    let f = |x: f64| x.powi(2);
    let x = 2.0;
    let h = 1e-6;
    let derivative = numerical_derivative(f, x, h);
    println!(
        "The numerical derivative of x^2 at x = {} is approximately {}",
        x, derivative
    );
}
