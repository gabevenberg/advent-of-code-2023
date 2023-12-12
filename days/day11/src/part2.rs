use crate::parse::*;

pub fn part2(input: &SparseSpace) -> usize {
    input.get_sum_of_distances(999_999)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = concat!(
            "...#......\n",
            ".......#..\n",
            "#.........\n",
            "..........\n",
            "......#...\n",
            ".#........\n",
            ".........#\n",
            "..........\n",
            ".......#..\n",
            "#...#.....\n",
        );
        assert_eq!(parse(input).get_sum_of_distances(9), 1030);
        assert_eq!(parse(input).get_sum_of_distances(99), 8410);
    }
}
