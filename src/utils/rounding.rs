use super::parsing::to;

#[inline]
pub fn round_one_digit_precision(x: f64) -> f64 {
    to(&format!("{:.1$}", x, 1))
}
