#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// range that includes the start, but excludes the end.
pub struct Range<T>
where
    T: PartialOrd + Ord + PartialEq + Copy,
{
    start: T,
    end: T,
}

impl<T> Range<T>
where
    T: PartialOrd + Ord + PartialEq + Copy,
{
    pub fn new(start: T, end: T) -> Self {
        Self {
            start: start.min(end),
            end: end.max(start),
        }
    }

    /// calcs whether self and other overlap at all. symettric.
    pub fn any_overlap(&self, other: &Self) -> bool {
        self.start < other.end && self.end > other.start
    }

    /// calculates the range that is part of both ranges.
    /// Returns None if the ranges do not overlap.
    pub fn calc_intersection(&self, other: &Self) -> Option<Self> {
        if self.any_overlap(other) {
            Some(
                Self::new(
                    self.start.max(other.start), self.end.min(other.end)
                )
            )
        } else {
            None
        }
    }

    ///calcs whether self completely contains other
    pub fn complete_overlap(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn in_range(&self, other: T) -> bool {
        self.start <= other && other < self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_intersection() {
        let a = Range::new(1, 5);
        let b = Range::new(3, 9);
        let c = Range::new(4, 6);
        let d = Range::new(6, 8);
        let e = Range::new(1, 4);
        assert_eq!(a.calc_intersection(&b), Some(Range::new(3,5)));
        assert_eq!(b.calc_intersection(&a), Some(Range::new(3,5)));

        assert_eq!(a.calc_intersection(&d), None);
        assert_eq!(d.calc_intersection(&a), None);

        assert_eq!(c.calc_intersection(&b), Some(c));
        assert_eq!(b.calc_intersection(&c), Some(c));

        assert_eq!(e.calc_intersection(&b), Some(Range::new(3,4)));
        assert_eq!(b.calc_intersection(&e), Some(Range::new(3,4)));
    }

    #[test]
    fn test_any_overlap() {
        let a = Range::new(1, 5);
        let b = Range::new(3, 9);
        let c = Range::new(4, 6);
        let d = Range::new(6, 8);
        let e = Range::new(1, 4);
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
        let a = Range::new(1, 5);
        let b = Range::new(3, 9);
        let c = Range::new(4, 6);
        let d = Range::new(6, 8);
        let e = Range::new(1, 4);
        assert!(a.complete_overlap(&a));

        assert!(a.complete_overlap(&e));
        assert!(!e.complete_overlap(&a));

        assert!(b.complete_overlap(&c));
        assert!(!c.complete_overlap(&b));

        assert!(b.complete_overlap(&d));
        assert!(!d.complete_overlap(&b));
    }
}
