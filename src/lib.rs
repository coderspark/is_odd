pub trait IsOdd {
    fn is_odd(&self) -> bool;
}

impl IsOdd for i8 {
    fn is_odd(&self) -> bool {
        self % 2 == 0
    }
}

impl IsOdd for i16 {
    fn is_odd(&self) -> bool {
        self % 2 == 0
    }
}

impl IsOdd for i32 {
    fn is_odd(&self) -> bool {
        self % 2 == 0
    }
}

impl IsOdd for i64 {
    fn is_odd(&self) -> bool {
        self % 2 == 0
    }
}

impl IsOdd for i128 {
    fn is_odd(&self) -> bool {
        self % 2 == 0
    }
}

impl IsOdd for isize {
    fn is_odd(&self) -> bool {
        self % 2 == 0
    }
}

impl IsOdd for f32 {
    fn is_odd(&self) -> bool {
        *self as isize % 2 == 0
    }
}

impl IsOdd for f64 {
    fn is_odd(&self) -> bool {
        *self as isize % 2 == 0
    }
}

impl IsOdd for u8 {
    fn is_odd(&self) -> bool {
        self % 2 == 0
    }
}

impl IsOdd for u16 {
    fn is_odd(&self) -> bool {
        self % 2 == 0
    }
}

impl IsOdd for u32 {
    fn is_odd(&self) -> bool {
        self % 2 == 0
    }
}

impl IsOdd for u64 {
    fn is_odd(&self) -> bool {
        self % 2 == 0
    }
}

impl IsOdd for u128 {
    fn is_odd(&self) -> bool {
        self % 2 == 0
    }
}

impl IsOdd for usize {
    fn is_odd(&self) -> bool {
        self % 2 == 0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_odd_i32() {
        assert_eq!(5.is_odd(), true);
        assert_eq!(4.is_odd(), false);
        assert_eq!(0.is_odd(), false); // Explicitly test zero
        assert_eq!((-3).is_odd(), true);
        assert_eq!((-4).is_odd(), false);
    }

    #[test]
    fn test_is_odd_u32() {
        assert_eq!(5u32.is_odd(), true);
        assert_eq!(4u32.is_odd(), false);
        assert_eq!(0u32.is_odd(), false);
    }

    #[test]
    fn test_is_odd_i64() {
        assert_eq!(7i64.is_odd(), true);
        assert_eq!(8i64.is_odd(), false);
        assert_eq!((-1i64).is_odd(), true);
    }

    #[test]
    fn test_is_odd_u64() {
        assert_eq!(11u64.is_odd(), true);
        assert_eq!(10u64.is_odd(), false);
        assert_eq!(0u64.is_odd(), false);
    }
}
