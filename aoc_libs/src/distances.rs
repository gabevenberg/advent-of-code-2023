pub trait Distances{
    fn taxicab_distance(&self, other: &Self)->usize;
}
