use crate::parse::*;

pub fn part2(input: Race) -> u64 {
    input.num_ways_to_win()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = Race{ time: 71530, record: 940200 };
        assert_eq!(part2(input), 71503);
    }
}
