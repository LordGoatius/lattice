#![allow(unused)]

use matrix::Matrix;
use rand::Rng;
use traits::RingElement;
pub mod matrix;
pub mod traits;
pub mod vector;

//== Extension Trait ==//
pub trait Zip<T: RingElement, const N: usize> {
    fn zip(self, rhs: [T; N]) -> [(T, T); N];
}

impl<T: RingElement, const N: usize> Zip<T, N> for [T; N] {
    fn zip(self, rhs: [T; N]) -> [(T, T); N] {
        let mut ret: [(T, T); N] = [const { std::mem::MaybeUninit::uninit() }; N].map(|x| unsafe { x.assume_init() });
        for i in 0..N {
            ret[i].0 = self[i];
            ret[i].1 = rhs[i];
        }
        ret
    }
}

#[cfg(test)]
pub mod test {
}
