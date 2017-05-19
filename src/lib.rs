#![no_std]

use core::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct U9(u16);

impl U9 {
    #[inline]
    pub fn from_u16_masked(val: u16) -> Self {
        U9(val & 0x1FF)
    }

    #[inline]
    pub fn try_from_u16(val: u16) -> Option<Self> {
        match val {
            0...0x1FF => Some(U9(val)),
            _ => None,
        }
    }
}

// TODO: other impls
impl fmt::Debug for U9 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <u16 as fmt::Debug>::fmt(&self.0, f)
    }
}

impl fmt::Display for U9 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <u16 as fmt::Display>::fmt(&self.0, f)
    }
}

impl From<u8> for U9 {
    #[inline]
    fn from(val: u8) -> Self {
        U9(val.into())
    }
}

impl From<U9> for u16 {
    #[inline]
    fn from(val: U9) -> Self {
        val.0.into()
    }
}

impl From<U9> for i16 {
    #[inline]
    fn from(val: U9) -> Self {
        // this will never overflow because U9 maintains invariant self.0 < 2^9
        val.0 as i16
    }
}

impl From<U9> for u32 {
    #[inline]
    fn from(val: U9) -> Self {
        val.0.into()
    }
}

impl From<U9> for i32 {
    #[inline]
    fn from(val: U9) -> Self {
        val.0.into()
    }
}

impl From<U9> for u64 {
    #[inline]
    fn from(val: U9) -> Self {
        val.0.into()
    }
}

impl From<U9> for i64 {
    #[inline]
    fn from(val: U9) -> Self {
        val.0.into()
    }
}

#[cfg(test)]
mod tests {
    use ::U9;
    use ::core::fmt::Debug;

    fn check_conversion<T: From<U9> + Eq + Debug>(u9: U9, val: T) {
        let u9: T = u9.into();
        assert_eq!(u9, val);
    }

    #[test]
    fn conversions() {
        check_conversion(0u8.into(), 0u16);
        check_conversion(42u8.into(), 42u16);
        check_conversion(U9::from_u16_masked(0u16), 0u16);
        check_conversion(U9::from_u16_masked(42u16), 42u16);
        check_conversion(U9::from_u16_masked(511), 511u16);
        check_conversion(U9::from_u16_masked(512), 0u16);

        check_conversion(U9::try_from_u16(0u16).unwrap(), 0u16);
        check_conversion(U9::try_from_u16(42u16).unwrap(), 42u16);
        check_conversion(U9::try_from_u16(511).unwrap(), 511u16);

        assert!(U9::try_from_u16(512).is_none());
    }
}
