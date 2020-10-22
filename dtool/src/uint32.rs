use std::ops::{Add, AddAssign, BitXorAssign, Shl, Shr, Sub, SubAssign};

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct Uint32(pub u32);

impl From<u32> for Uint32 {
    fn from(v: u32) -> Self {
        Self(v)
    }
}

impl From<usize> for Uint32 {
    fn from(v: usize) -> Self {
        Self(v as u32)
    }
}

impl From<u8> for Uint32 {
    fn from(v: u8) -> Self {
        Self(v as u32)
    }
}

impl From<i32> for Uint32 {
    fn from(v: i32) -> Self {
        Self(v as u32)
    }
}

impl Add for Uint32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.overflowing_add(rhs.0).0)
    }
}

impl AddAssign for Uint32 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl AddAssign<usize> for Uint32 {
    fn add_assign(&mut self, rhs: usize) {
        *self = Self(self.0 + rhs as u32)
    }
}

impl Sub for Uint32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.overflowing_sub(rhs.0).0)
    }
}

impl SubAssign for Uint32 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Shl<u32> for Uint32 {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        Self(self.0.overflowing_shl(rhs).0)
    }
}

impl Shr<u32> for Uint32 {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        Self(self.0.overflowing_shr(rhs).0)
    }
}

impl BitXorAssign for Uint32 {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 ^ rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Uint32::from(1);
        let b = Uint32::from(2);
        assert_eq!(a + b, Uint32(3));
    }

    #[test]
    fn test_add_assign() {
        let mut a = Uint32::from(1);
        let b = Uint32::from(2);
        a += b;
        assert_eq!(a, Uint32(3));
    }

    #[test]
    fn test_sub() {
        let a = Uint32::from(1);
        let b = Uint32::from(2);
        assert_eq!(b - a, Uint32(1));
    }

    #[test]
    fn test_sub_assign() {
        let a = Uint32::from(1);
        let mut b = Uint32::from(2);
        b -= a;
        assert_eq!(b, Uint32(1));
    }

    #[test]
    fn test_bitxor_assign() {
        let mut a = Uint32::from(1);
        let b = Uint32::from(2);
        a ^= b;
        assert_eq!(a, Uint32(3));
    }

}
