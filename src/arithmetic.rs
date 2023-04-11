pub const EPSILON: f64 = 0.00001;

pub fn fuzzy_equal(a: f64, b: f64) -> bool
{
    let diff = a - b;
    diff.abs() < EPSILON
}
