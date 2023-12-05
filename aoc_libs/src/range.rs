#[derive(Debug, PartialEq, Eq)]
pub struct InclusiveRange<T>
where
    T: PartialOrd + Ord + PartialEq + Copy,
{
    start: T,
    end: T,
}

impl<T> InclusiveRange<T>
where
    T: PartialOrd + Ord + PartialEq + Copy,
{
    pub fn new(start: T, end: T) -> Self {
        Self {
            start: start.min(end),
            end: end.max(start),
        }
    }

    pub fn any_overlap(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    pub fn calc_overlap(&self, other: &Self) -> Self {
        let overlap_start = self.start.min(other.start);
        let overlap_end = self.end.max(other.end);
        InclusiveRange::new(overlap_start, overlap_end)
    }

    pub fn complete_overlap(&self, other: &Self) -> bool {
        self.calc_overlap(other) == *self || self.calc_overlap(other) == *other
    }

    pub fn in_range(&self, other: T) -> bool {
        self.start <= other && other <= self.end
    }
}
