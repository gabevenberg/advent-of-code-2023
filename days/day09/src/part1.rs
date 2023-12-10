use crate::parse::*;

pub fn part1(input: &[Vec<i32>]) -> i32 {
    input.iter().map(|l| extrapolate_sequence(l)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];
        assert_eq!(part1(&input), 114);
    }
}
