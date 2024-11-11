use crate::traits::RingElement;

pub mod binops;
pub mod det;

// M x N means M rows and N columns
#[derive(Debug, Clone, Copy, PartialEq)]
struct Matrix<T: RingElement, const M: usize, const N: usize>([[T; N]; M]);

// M rows N columns
impl<T: RingElement, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn transpose(self) -> Matrix<T, N, M> {
        let mut ret: Matrix<T, N, M> = Matrix([[T::default(); M]; N]);
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
