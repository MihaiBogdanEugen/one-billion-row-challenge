use std::fmt::Debug;
use std::str::FromStr;

#[inline]
pub fn to<T>(text: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    FromStr::from_str(text.trim()).unwrap()
}
