// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// /// range that includes the start, but excludes the end.
// pub struct Range<T>
// where
//     T: PartialOrd + Ord + PartialEq + Copy,
// {
//     start: T,
//     end: T,
// }

use std::ops::Range;

pub trait RangeIntersection {
    fn any_overlap(&self, other: &Self) -> bool;
    fn calc_intersection(&self, other: &Self) -> Option<Self>
    where
        Self: std::marker::Sized;
    fn complete_overlap(&self, other: &Self) -> bool;
}

impl<T> RangeIntersection for Range<T>
where
    T: PartialOrd + Ord + PartialEq + Copy,
{
    /// calcs whether self and other overlap at all. symettric.
    fn any_overlap(&self, other: &Self) -> bool {
        if self.is_empty() || other.is_empty() {
            false
        } else {
            self.start < other.end && self.end > other.start
        }
    }

    /// calculates the range that is part of both ranges.
    /// Returns None if the ranges do not overlap.
    fn calc_intersection(&self, other: &Self) -> Option<Self> {
        if self.any_overlap(other) {
            Some(self.start.max(other.start)..self.end.min(other.end))
        } else {
            None
        }
    }

    ///calcs whether self completely contains other
    fn complete_overlap(&self, other: &Self) -> bool {
        if self.is_empty() || other.is_empty() {
            false
        } else {
            self.start <= other.start && self.end >= other.end
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_intersection() {
        let a = 1..5;
        let b = 3..9;
        let c = 4..6;
        let d = 6..8;
        let e = 1..4;
        assert_eq!(a.calc_intersection(&b), Some(3..5));
        assert_eq!(b.calc_intersection(&a), Some(3..5));

        assert_eq!(a.calc_intersection(&d), None);
        assert_eq!(d.calc_intersection(&a), None);

        assert_eq!(c.calc_intersection(&b), Some(c.clone()));
        assert_eq!(b.calc_intersection(&c), Some(c.clone()));

        assert_eq!(e.calc_intersection(&b), Some(3..4));
        assert_eq!(b.calc_intersection(&e), Some(3..4));
    }

    #[test]
    fn test_any_overlap() {
        let a = 1..5;
        let b = 3..9;
        let c = 4..6;
        let d = 6..8;
        let e = 1..4;
        assert!(a.any_overlap(&b));
        assert!(b.any_overlap(&a));

        assert!(b.any_overlap(&c));
        assert!(c.any_overlap(&b));

        assert!(!d.any_overlap(&a));
        assert!(!a.any_overlap(&d));

        assert!(e.any_overlap(&b));
        assert!(b.any_overlap(&e));

        assert!(!e.any_overlap(&d));
        assert!(!d.any_overlap(&e));
    }
    #[test]
    fn test_complete_overlap() {
        let a = 1..5;
        let b = 3..9;
        let c = 4..6;
        let d = 6..8;
        let e = 1..4;
        assert!(a.complete_overlap(&a));

        assert!(a.complete_overlap(&e));
        assert!(!e.complete_overlap(&a));

        assert!(b.complete_overlap(&c));
        assert!(!c.complete_overlap(&b));

        assert!(b.complete_overlap(&d));
        assert!(!d.complete_overlap(&b));
    }
}
