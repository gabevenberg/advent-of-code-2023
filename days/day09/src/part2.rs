use crate::parse::*;

pub fn part2(input: &mut [Vec<i32>]) -> i32 {
    input
        .iter_mut()
        .map(|l| {
            l.reverse();
            extrapolate_sequence(l)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let mut input = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];
        assert_eq!(part2(&mut input), 2);
    }
}
