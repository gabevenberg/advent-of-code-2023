use crate::parse::*;

pub fn part1(input: &[Race]) -> u64 {
    input.iter().map(|r| r.num_ways_to_win()).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            Race { time: 7, record: 9 },
            Race {
                time: 15,
                record: 40,
            },
            Race {
                time: 30,
                record: 200,
            },
        ];
        assert_eq!(part1(&input), 288);
    }
}
