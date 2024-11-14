use std::ops::{Add, Deref, DerefMut, Mul, Sub};

use crate::matrix::Matrix;
use crate::traits::RingElement;
use crate::vector::Vector;

//== Deref ==//
impl<T: RingElement, const M: usize, const N: usize> Deref for Matrix<T, M, N> {
    type Target = [[T; N]; M];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//== DerefMut ==//
impl<T: RingElement, const M: usize, const N: usize> DerefMut for Matrix<T, M, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

//== Matrix Multiplication ==//
// M x N means M rows and N columns
// N x K means N rows and K columns
// (M x N) * (N x K) = (M x K) (M rows, K columns)
impl<T: RingElement, const M: usize, const N: usize, const K: usize> Mul<Matrix<T, N, K>>
    for Matrix<T, M, N>
{
    type Output = Matrix<T, M, K>;
    fn mul(self, rhs: Matrix<T, N, K>) -> Self::Output {
        let mut prod: Matrix<T, M, K> = Matrix([[T::default(); K]; M]);
        // create every row
        for row in 0..M {
            for col in 0..K {
                let mut sum: T = T::default();
                for sum_index in 0..N {
                    sum = sum + self[row][sum_index] * rhs[sum_index][col];
                }
                prod[row][col] = sum;
            }
        }
        prod
    }
}

//== Scalar Multiplication ==//
impl<T: RingElement, const M: usize, const N: usize> Mul<T> for Matrix<T, M, N> {
    type Output = Matrix<T, M, N>;
    fn mul(self, rhs: T) -> Self::Output {
        Matrix(self.map(|arr| arr.map(|x| x * rhs)))
    }
}

//== Vector Multiplication ==//
// M x N means M rows and N columns
impl<T: RingElement, const M: usize, const N: usize> Mul<Vector<T, N>> for Matrix<T, M, N> {
    type Output = Vector<T, M>;
    fn mul(self, rhs: Vector<T, N>) -> Self::Output {
        let mut prod: Vector<T, M> = Vector([T::default(); M]);
        // create every element
        for row in 0..M {
            let mut sum: T = T::default();
            for sum_index in 0..N {
                sum = sum + self[row][sum_index] * rhs[sum_index];
            }
            prod[row] = sum;
        }
        prod
    }
}

//== Matrix Addition ==//
impl<T: RingElement, const M: usize, const N: usize> Add for Matrix<T, M, N> {
    type Output = Matrix<T, M, N>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut arr: [[T; N]; M] =
            [[ const { std::mem::MaybeUninit::uninit() }; N].map(|x| unsafe { x.assume_init() }); M];
        arr.iter_mut().enumerate().for_each(|(i, val)| {
            val.iter_mut()
                .enumerate()
                .for_each(|(j, val)| *val = self[i][j] + rhs[i][j])
        });
        Matrix(arr)
    }
}

//== Matrix Subtraction ==//
impl<T: RingElement, const M: usize, const N: usize> Sub for Matrix<T, M, N> {
    type Output = Matrix<T, M, N>;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut arr: [[T; N]; M] =
            [[ const { std::mem::MaybeUninit::uninit() }; N].map(|x| unsafe { x.assume_init() }); M];
        arr.iter_mut().enumerate().for_each(|(i, val)| {
            val.iter_mut()
                .enumerate()
                .for_each(|(j, val)| *val = self[i][j] - rhs[i][j])
        });
        Matrix(arr)
    }
}

#[cfg(test)]
pub mod test {
    use crate::matrix::Matrix;
    #[test]
    fn test_mul() {
        let ident: Matrix<i32, 2, 2> = Matrix([[1, 0], [0, 1]]);

        let mat: Matrix<i32, 2, 2> = Matrix([[5, 6], [7, 8]]);

        assert_eq!(mat, ident * mat);
        assert_eq!(mat, mat * ident);

        // Two rows, three columns
        let twobythree: Matrix<i32, 2, 3> = Matrix([[1, 0, 2], [0, 1, 2]]);

        // Three rows, two columns
        let threebytwo: Matrix<i32, 3, 2> = Matrix([[5, 6], [7, 8], [7, 8]]);
        let two_times_threebytwo: Matrix<i32, 3, 2> = Matrix([[10, 12], [14, 16], [14, 16]]);

        assert_eq!(two_times_threebytwo, threebytwo * 2);

        assert_eq!(
            threebytwo * twobythree,
            Matrix([[5, 6, 22], [7, 8, 30], [7, 8, 30]])
        );
        assert_eq!(twobythree * threebytwo, Matrix([[19, 22], [21, 24]]));
    }
}
