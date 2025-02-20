extern crate fast_floats;

use fast_floats::Fast;

// for demonstration purposes
pub unsafe fn fast_sum(xs: &[f64]) -> f64 {
    *xs.iter()
        .map(|&x| Fast::new(x))
        .fold(Fast::new(0.), |acc, x| acc + x)
}

// for demonstration purposes
pub unsafe fn fast_dot(xs: &[f64], ys: &[f64]) -> f64 {
    *xs.iter().zip(ys).fold(Fast::new(0.), |acc, (&x, &y)| {
        acc + Fast::new(x) * Fast::new(y)
    })
}

pub fn regular_sum(xs: &[f64]) -> f64 {
    xs.iter().map(|&x| x).fold(0., |acc, x| acc + x)
}

fn main() {}
