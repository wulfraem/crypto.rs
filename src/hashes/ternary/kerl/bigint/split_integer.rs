// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Split integers to access the high and low parts of an integer.

/// Represents a split integer into high and low parts.
pub(crate) trait SplitInteger: Copy {
    /// The type of the high part of the integer.
    type High;
    /// The type of the low part of the integer.
    type Low;

    /// Returns the high part of the integer.
    fn hi(self) -> Self::High;
    /// Returns the low part of the integer.
    fn lo(self) -> Self::Low;
}

impl SplitInteger for i64 {
    type High = i32;
    type Low = u32;

    #[allow(clippy::cast_possible_truncation)] // Truncation is used to remove the low part of the integer.
    fn hi(self) -> Self::High {
        (self >> 32) as Self::High
    }

    #[allow(clippy::cast_possible_truncation)] // Truncation is used to remove the high part of the integer.
    #[allow(clippy::cast_sign_loss)] // Sign loss is expected as `Self::Low` represents the lower part of the integer.
    fn lo(self) -> Self::Low {
        self as Self::Low
    }
}

impl SplitInteger for u64 {
    type High = u32;
    type Low = u32;

    #[allow(clippy::cast_possible_truncation)] // Truncation is used to remove the low part of the integer.
    fn hi(self) -> Self::High {
        (self >> 32) as Self::High
    }

    #[allow(clippy::cast_possible_truncation)] // Truncation is used to remove the high part of the integer.
    fn lo(self) -> Self::Low {
        self as Self::Low
    }
}

#[cfg(test)]
mod tests {
    use super::SplitInteger;

    macro_rules! test_split_integers {
        ( $( [$fname:ident, $src:expr, $dst:expr] ),+ $(,)? ) => {
            $(
                #[test]
                fn $fname() {
                    assert_eq!($src, $dst);
                }
            )+
        }
    }

    test_split_integers!(
        // i64
        [split_i64_hi_one_is_zero, 1i64.hi(), 0i32],
        [split_i64_lo_one_is_one, 1i64.lo(), 1u32],
        [split_i64_hi_max_is_max, i64::MAX.hi(), i32::MAX],
        [split_i64_lo_max_is_max, i64::MAX.lo(), u32::MAX],
        [split_i64_hi_min_is_min, i64::MIN.hi(), i32::MIN],
        [split_i64_lo_min_is_zero, i64::MIN.lo(), 0u32],
        [split_i64_hi_neg_one_is_neg_one, (-1i64).hi(), -1i32],
        [split_i64_lo_neg_one_is_max, (-1i64).lo(), u32::MAX],
        // u64
        [split_u64_hi_one_is_zero, 1u64.hi(), 0u32],
        [split_u64_lo_one_is_one, 1u64.lo(), 1u32],
        [split_u64_hi_max_is_max, u64::MAX.hi(), u32::MAX],
        [split_u64_lo_max_is_max, u64::MAX.lo(), u32::MAX],
        [split_u64_hi_min_is_min, u64::MIN.hi(), 0u32],
        [split_u64_lo_min_is_zero, u64::MIN.lo(), 0u32],
    );
}
