use std::ops::{Add, Deref, DerefMut, Mul, Sub};

use rand::Rng;

use crate::{traits::RingElement, Zip};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<T: RingElement, const N: usize>(pub [T; N]);

//== Deref ==//
impl<T: RingElement, const N: usize> Deref for Vector<T, N> {
    type Target = [T; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//== DerefMut ==//
impl<T: RingElement, const N: usize> DerefMut for Vector<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

//== Vector Addition ==//
impl<T: RingElement, const N: usize> Add for Vector<T, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.zip(*rhs).map(|(l, r)| l + r))
    }
}

//== Vector Subtraction ==//
impl<T: RingElement, const N: usize> Sub for Vector<T, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.zip(*rhs).map(|(l, r)| l - r))
    }
}

//== Scalar Multiplication ==//
impl<T: RingElement, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Vector<T, N>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector(self.map(|x| x * rhs))
    }
}

impl<const N: usize> Vector<isize, N> {
    pub fn to_usize(self) -> Vector<usize, N> {
        Vector(self.map(|elem| elem as usize))
    }
}

impl<const N: usize> Vector<usize, N> {
    pub fn to_isize(self) -> Vector<isize, N> {
        Vector(self.map(|elem| elem as isize))
    }
}

impl<T: RingElement, const N: usize> Vector<T, N> {
    pub fn dot(self, rhs: Vector<T, N>) -> T {
        (0..N).fold(T::default(), |acc, i| acc + (self[i] * rhs[i]))
    }
    // Gotta add cross and dot product
}

#[cfg(test)]
pub mod tests {
    use crate::vector::Vector;

    #[test]
    fn test_const() {
        let one_d: Vector<u64, 1> = Vector([1]);
        let big_d: Vector<u64, 1024> = Vector([7; 1024]);
        let big_d_1: Vector<u64, 1024> = Vector([1; 1024]);

        let type_0 = std::any::type_name_of_val(&one_d);
        let type_1 = std::any::type_name_of_val(&big_d_1);

        let thing = big_d + big_d_1;
        let thing0 = big_d - big_d_1;
        assert_eq!(thing, Vector([8; 1024]));
        assert_eq!(thing0, Vector([6; 1024]));
        println!("{type_0:?}");
        println!("{type_1:?}");
    }
}
