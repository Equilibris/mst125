use std::ops::{Add, Div, Neg, Rem, Sub};

use num_traits::{CheckedDiv, NumCast, One, Pow, Zero};

use crate::algebra::Algebra;

pub trait Gcdd = CheckedDiv<Output = Self>
    + Div<Self, Output = Self>
    + Rem<Self, Output = Self>
    + Add<Self, Output = Self>
    + One
    + Zero
    + Eq
    + PartialOrd<Self>
    + Copy
    + Neg<Output = Self>;

#[derive(Clone, Debug)]
pub struct GcdEntry<T: Gcdd> {
    pub a: T,
    pub quotient: T,
    pub b: T,
    pub remainder: T,
}

#[inline(always)]
pub fn least_res<T: Gcdd>(base: T, modulo: T) -> T {
    ((base % modulo) + modulo) % modulo
}

pub fn gcd<T: Gcdd>(a: T, b: T) -> Option<(T, Vec<GcdEntry<T>>)> {
    let (a, b) = if a > b { (a, b) } else { (b, a) };

    let mut out = vec![GcdEntry {
        a,
        quotient: a.checked_div(&b)?,
        b,
        remainder: a % b,
    }];

    while out.last().unwrap().remainder != T::zero() {
        let last = out.last().unwrap();

        let (a, b) = (last.b, last.remainder);

        out.push(GcdEntry {
            a,
            quotient: a / b,
            b,
            remainder: a % b,
        });
    }

    Some(if out.len() == 1 {
        (b, out)
    } else {
        (out.iter().nth_back(1).unwrap().remainder, out)
    })
}

pub fn fast_modular_exponentiation<
    E: Rem<E, Output = E> + NumCast,
    T: Pow<E, Output = T> + NumCast + Sub<T, Output = T> + Rem<T, Output = T> + One + Copy,
>(
    base: T,
    exp: E,
    prime: T,
) -> Option<T> {
    Some(T::pow(base % prime, exp % E::from(prime - T::one())?) % prime)
}

pub fn bezout_with_gcd<T: Gcdd>(a: T, b: T, (gcd, m): (T, Vec<GcdEntry<T>>)) -> (T, T) {
    use Algebra::*;
    let mut x: Algebra<T, T> = Var(gcd);

    let mut m = m.into_iter().rev();

    m.next();

    for GcdEntry {
        a,
        quotient,
        b,
        remainder,
    } in m
    {
        x.var_sub(
            &remainder,
            &Add(box Var(a), box Mul(box Val(-quotient), box Var(b))),
        );
    }

    let mut x = x.into_zip_sequence().into_iter();

    if x.len() == 1 {
        #[inline(always)]
        fn zow<T: Zero + One>(v: bool) -> T {
            if v {
                T::one()
            } else {
                T::zero()
            }
        }
        (zow(a == gcd), zow((b == gcd) && !(a == gcd)))
    } else {
        let u;
        let v;

        let (i, j) = x.next().unwrap();

        if i.into_iter().next().unwrap() == a {
            u = j;
            v = x.next().unwrap().1;
        } else {
            v = j;
            u = x.next().unwrap().1;
        }

        (u, v)
    }
}

pub fn bezout<T: Gcdd>(a: T, b: T) -> Option<(T, T)> {
    Some(bezout_with_gcd(a, b, gcd(a, b)?))
}

pub fn mul_inv_with_gcd<T: Gcdd>(base: T, modulo: T, (gcd, m): (T, Vec<GcdEntry<T>>)) -> Option<T> {
    if gcd == T::one() {
        let (a, _) = bezout_with_gcd(base, modulo, (gcd, m));

        Some(least_res(a, modulo))
    } else {
        None
    }
}
pub fn mul_inv<T: Gcdd>(base: T, modulo: T) -> Option<T> {
    gcd(base, modulo).and_then(move |x| mul_inv_with_gcd(base, modulo, x))
}

// Solves Lin-cong of the form $a ~ x \equiv b$
pub fn lin_cong(a: i64, b: i64, n: i64) -> Option<i64> {
    let (gcd, m) = gcd(a, n)?;

    if gcd == 1 {
        Some(least_res(mul_inv_with_gcd(a, n, (gcd, m))? * b, n))
    } else if b % gcd == 0 {
        lin_cong(a % gcd, b % gcd, n % gcd)
    } else {
        None
    }
}

pub fn prepare_affine(a: impl Iterator<Item = char>) -> impl Iterator<Item = u8> {
    a.filter_map(|x| match x {
        'A'..='Z' => Some(x as u8 - 'A' as u8),
        'a'..='z' => Some(x as u8 - 'a' as u8),
        _ => None,
    })
}
pub fn restore_affine(a: impl Iterator<Item = u8>) -> impl Iterator<Item = char> {
    a.filter(|x| (0..26).contains(x))
        .map(|x| ('A' as u8 + x) as char)
}

pub struct Affine {
    pub(self) mul: i8,
    pub(self) inv: i8,
    pub(self) off: i8,
}
pub struct EAffine<'a, T: Iterator<Item = u8>>(&'a Affine, T);

impl<'a, T: Iterator<Item = u8>> Iterator for EAffine<'a, T> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.1
            .next()
            .map(|x| ((((self.0.mul as i16) * (x as i16)) + self.0.off as i16) % 26) as u8)
    }
}
pub struct DAffine<'a, T: Iterator<Item = u8>>(&'a Affine, T);

impl<'a, T: Iterator<Item = u8>> Iterator for DAffine<'a, T> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.1
            .next()
            .map(|x| least_res((self.0.inv as i16) * ((x as i16) - (self.0.off as i16)), 26) as u8)
    }
}

impl Affine {
    pub fn new(mul: i8, off: i8) -> Option<Self> {
        Some(Self {
            off,
            mul,
            inv: mul_inv(mul, 26)?,
        })
    }
}
impl<'a> Affine {
    pub fn decrypt<Stream: Iterator<Item = u8>>(&'a self, s: Stream) -> DAffine<'a, Stream> {
        DAffine(self, s)
    }
    pub fn encrypt<Stream: Iterator<Item = u8>>(&'a self, s: Stream) -> EAffine<'a, Stream> {
        EAffine(self, s)
    }

    pub fn e_str<'b: 'a>(&'a self, s: &'b str) -> EAffine<'a, impl 'b + Iterator<Item = u8>> {
        self.encrypt(prepare_affine(s.chars()))
    }
    pub fn d_str<'b: 'a, Stream: 'b + Iterator<Item = u8>>(&'a self, s: Stream) -> String {
        restore_affine(self.decrypt(s)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcdt() {
        assert_eq!(8, gcd(72, 32).unwrap().0);
        assert!(gcd(72, 0).is_none());
    }

    #[test]
    fn fltt() {
        assert_eq!(fast_modular_exponentiation(4, 20u32, 7).unwrap(), 2);
        assert_eq!(fast_modular_exponentiation(5, 6u32, 7).unwrap(), 1);
        assert_eq!(fast_modular_exponentiation(18, 18u32, 7).unwrap(), 1);
    }

    #[test]
    fn benzoutt() {
        assert_eq!((8, -15), bezout(32, 17).unwrap());
        assert_eq!((-15, 8), bezout(17, 32).unwrap());
        assert_eq!((1, 0), bezout(17, 17).unwrap());
        assert_eq!((-9, 34), bezout(185, 49).unwrap());
        assert_eq!((5, -11), bezout(93, 42).unwrap());
        assert_eq!((-12, 29), bezout(70, 29).unwrap());
    }

    #[test]
    fn mul_invt() {
        assert_eq!(1, mul_inv(1, 9).unwrap());
        assert_eq!(2, mul_inv(5, 9).unwrap());
        assert_eq!(8, mul_inv(5, 13).unwrap());
    }

    #[test]
    fn lin_congt() {
        assert_eq!(6, lin_cong(5, 21, 9).unwrap());
        assert_eq!(4, lin_cong(11, 6, 38).unwrap());
        assert!(lin_cong(21, 14, 30).is_none());
        assert!(lin_cong(8, 24, 28).is_none());
    }

    #[test]
    fn affinet() {
        let affine = Affine::new(7, 12).unwrap();

        let s = prepare_affine("IQZC".chars()).collect::<Vec<_>>();

        println!("{:?}", s);

        let dec = affine.d_str(s.into_iter());

        println!("{}", dec);
    }
}
