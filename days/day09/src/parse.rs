pub fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect()
}

pub fn get_differences(input: &[i32]) -> Vec<i32> {
    input.windows(2).map(|p| p[1] - p[0]).collect()
}

pub fn extrapolate_sequence(input: &[i32]) -> i32 {
    if input.iter().all(|i| *i == 0) {
        return 0;
    };
    let diffs = get_differences(input);
    input.last().unwrap() + extrapolate_sequence(&diffs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_differences() {
        assert_eq!(get_differences(&[0, 3, 6, 9, 12, 15]), vec![3, 3, 3, 3, 3]);
        assert_eq!(get_differences(&[1, 3, 6, 10, 15, 21]), vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_extrapolate_sequence() {
        assert_eq!(extrapolate_sequence(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(extrapolate_sequence(&[1, 3, 6, 10, 15, 21]), 28);
    }

    #[test]
    fn test_parse() {
        let input = concat!("0 3 6 9 12 15\n", "1 3 6 10 15 21\n", "10 13 16 21 30 45\n",);
        assert_eq!(
            parse(input),
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![1, 3, 6, 10, 15, 21],
                vec![10, 13, 16, 21, 30, 45],
            ]
        );
    }
}
