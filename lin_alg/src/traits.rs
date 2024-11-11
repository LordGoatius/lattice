use std::ops::{Add, Div, Mul, Sub};

/// # Noteworthy
/// If Default is not the additive identity, this library will give odd results
/// Keep this in mind when using user-defined algebraic types, as this trait is auto implemented
/// for any type that implements all of the contained traits
pub trait RingElement:
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Clone + Copy + Default
{
}

/// # Noteworthy
/// If Default is not the additive identity, this library will give odd results
/// Keep this in mind when using user-defined algebraic types, as this trait is auto implemented
/// for any type that implements all of the contained traits
pub trait FieldElement: RingElement + Div<Output = Self> {}

impl<T> RingElement for T where
    T: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Clone + Copy + Default
{
}

impl<T> FieldElement for T where T: RingElement + Div<Output = Self> {}
