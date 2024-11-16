// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SaturatingU16(u16);

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let saturated = self.0.saturating_add(rhs.0);
        SaturatingU16(saturated)
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        SaturatingU16::add(self, SaturatingU16::from(rhs))
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16::add(self, SaturatingU16::from(rhs))
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: &u16) -> Self::Output {
        SaturatingU16::add(self, SaturatingU16::from(rhs))
    }
}

impl Add<&u8> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: &u8) -> Self::Output {
        SaturatingU16::add(self, SaturatingU16::from(rhs))
    }
}

impl From<u16> for SaturatingU16 {
    fn from(s: u16) -> Self {
        Self(s)
    }
}

impl From<u8> for SaturatingU16 {
    fn from(s: u8) -> Self {
        Self(s.into())
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(s: &u16) -> Self {
        Self(*s)
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(s: &u8) -> Self {
        Self((*s).into())
    }
}


impl From<&SaturatingU16> for SaturatingU16 {
    fn from(s: &SaturatingU16) -> Self {
        (*s).clone()
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}