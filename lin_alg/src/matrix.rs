use core::f32;

use rand_distr::{Distribution, Normal};

use crate::traits::RingElement;

pub mod binops;
pub mod det;

// M x N means M rows and N columns
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<T: RingElement, const M: usize, const N: usize>(pub [[T; N]; M]);

impl<const M: usize, const N: usize> Matrix<isize, M, N> {
    pub fn to_usize(self) -> Matrix<usize, M, N> {
        Matrix(self.map(|row| row.map(|elem| elem as usize)))
    }
}

impl<const M: usize, const N: usize> Matrix<usize, M, N> {
    pub fn to_isize(self) -> Matrix<isize, M, N> {
        Matrix(self.map(|row| row.map(|elem| elem as isize)))
    }
}

// M rows N columns
impl<T: RingElement, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn transpose(self) -> Matrix<T, N, M> {
        let mut ret: Matrix<T, N, M> =
            // Safe because I override every uninit value
            Matrix([[ const { std::mem::MaybeUninit::uninit() }; M].map(|x| unsafe { x.assume_init() }); N]);
        // rows in self
        for i in 0..M {
            // cols in self
            for j in 0..N {
                ret[j][i] = self[i][j];
            }
        }
        ret
    }
}

impl<const M: usize, const N: usize> Matrix<f64, M, N> {
    pub fn round(self) -> Matrix<usize, M, N> {
        Matrix(self.map(|row| row.map(|i| i.round() as usize)))
    }
}
