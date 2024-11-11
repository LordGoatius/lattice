use crate::traits::RingElement;

use super::Matrix;

impl<T: RingElement, const M: usize> Matrix<T, M, M> {
    pub fn det(self) -> T {
        todo!()
    }
}
