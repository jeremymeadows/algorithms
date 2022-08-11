use std::iter::Step;

use crate::BigInt;

impl Step for BigInt {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if let Ok(steps) = (end - start).try_into() {
            Some(steps)
        } else {
            None
        }
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Some(start + count)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        Some(start - count)
    }
}
