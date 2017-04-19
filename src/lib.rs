extern crate gmp;
extern crate num;

use std::ops::{AddAssign, Sub, Div, Mul};
use gmp::mpq::Mpq;
use gmp::mpz::Mpz;
use num::One;

/// The even-index Bernoulli numbers ([A000367](https://oeis.org/A000367) /
/// [A002445](https://oeis.org/A002445)).
///
/// Note: This is an infinite iterator.
///
///     # extern crate bernoulli_numbers;
///     extern crate gmp;
///
///     use bernoulli_numbers::EvenBernoulli;
///     use gmp::mpq::Mpq;
///
///     # fn main() {
///     let seq: Vec<_> = EvenBernoulli::default().take(8).collect();
///     assert_eq!(seq, [Mpq::from(1),
///                      Mpq::from(1) / Mpq::from(6),
///                      Mpq::from(-1) / Mpq::from(30),
///                      Mpq::from(1) / Mpq::from(42),
///                      Mpq::from(-1) / Mpq::from(30),
///                      Mpq::from(5) / Mpq::from(66),
///                      Mpq::from(-691) / Mpq::from(2730),
///                      Mpq::from(7) / Mpq::from(6)]);
///     # }
///
pub struct EvenBernoulli {
    i: i64,
    power: Mpz,
    zs: EulerUpDown<Mpz>,
}

impl Default for EvenBernoulli {
    fn default() -> Self {
        Self {
            i: Default::default(),
            power: One::one(),
            zs: Default::default(),
        }
    }
}

impl Iterator for EvenBernoulli {
    type Item = Mpq;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i = -(i + if i >= 0 { 2 } else { -2 });
        Some(if i == 0 {
            One::one()
        } else {
            let z = match self.zs.nth(1) {
                None => return None,
                Some(z) => z,
            };
            self.power *= Mpz::from(4);
            let a = &self.power;
            let b = a.pow(2);
            let i = Mpz::from(i);
            Mpq::from(i * z) / Mpq::from(a - b)
        })
    }
}

/// Euler up/down (“zigzag”) numbers ([A000111](https://oeis.org/A000111)).
///
/// Note: This is an infinite iterator.
///
///     use bernoulli_numbers::EulerUpDown;
///
///     let seq: Vec<u64> = EulerUpDown::default().take(8).collect();
///     assert_eq!(seq, [1, 1, 1, 2, 5, 16, 61, 272]);
///
pub struct EulerUpDown<T> {
    source: Vec<T>,
    sink: Vec<T>,
}

impl<T> Default for EulerUpDown<T> {
    fn default() -> Self {
        Self {
            source: Default::default(),
            sink: Default::default(),
        }
    }
}

impl<T: Clone + One + AddAssign> Iterator for EulerUpDown<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        std::mem::swap(&mut self.source, &mut self.sink);
        let mut accum = match self.source.pop() {
            None => One::one(),
            Some(accum) => {
                self.sink.push(accum.clone());
                accum
            }
        };
        let item = accum.clone();
        while let Some(i) = self.source.pop() {
            accum += i;
            self.sink.push(accum.clone());
        }
        self.sink.push(accum);
        Some(item)
    }
}