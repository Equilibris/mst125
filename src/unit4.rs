use std::cmp::{Ordering, PartialOrd};
use std::ops::{Mul, Sub};

use num_traits::NumCast;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ConicClass {
    Ellipse,
    Parabola,
    Hyperbola,
}

pub struct GeneralConic<T>(pub T, pub T, pub T, pub T, pub T, pub T);

impl<T> std::fmt::Display for GeneralConic<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub fn get_conic_class<T: Copy + NumCast + Mul<T, Output = T> + Sub<T, Output = T> + Ord>(
    c: &GeneralConic<T>,
) -> ConicClass {
    match (c.1 * c.1 - <T as NumCast>::from(4).unwrap() * c.0 * c.2).cmp(&T::from(0).unwrap()) {
        Ordering::Less => ConicClass::Ellipse,
        Ordering::Equal => ConicClass::Parabola,
        Ordering::Greater => ConicClass::Hyperbola,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i_cma41_8() {
        let conic = GeneralConic(25, -14, 25, -86, -22, -479);

        println!("conic")
    }
}
