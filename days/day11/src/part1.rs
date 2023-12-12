use crate::parse::*;

pub fn part1(input: &SparseSpace) -> usize {
    input.get_sum_of_distances(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
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
        assert_eq!(part1(&parse(input)), 374)
    }
}
