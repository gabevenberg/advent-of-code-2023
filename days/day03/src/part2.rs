use crate::parse::*;

pub fn part2(input: &StructuredInput) -> usize {
    input
        .1
        .iter()
        .filter_map(|symbol| symbol.gear_ratio(&input.0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = (
            vec![
                PartNumber {
                    number: 467,
                    x: 0..3,
                    y: 0,
                },
                PartNumber {
                    number: 114,
                    x: 5..8,
                    y: 0,
                },
                PartNumber {
                    number: 35,
                    x: 2..4,
                    y: 2,
                },
                PartNumber {
                    number: 633,
                    x: 6..9,
                    y: 2,
                },
                PartNumber {
                    number: 617,
                    x: 0..3,
                    y: 4,
                },
                PartNumber {
                    number: 58,
                    x: 7..9,
                    y: 5,
                },
                PartNumber {
                    number: 592,
                    x: 2..5,
                    y: 6,
                },
                PartNumber {
                    number: 755,
                    x: 6..9,
                    y: 7,
                },
                PartNumber {
                    number: 664,
                    x: 1..4,
                    y: 9,
                },
                PartNumber {
                    number: 598,
                    x: 5..8,
                    y: 9,
                },
            ],
            vec![
                Symbol {
                    x: 3,
                    y: 1,
                    char: '*',
                },
                Symbol {
                    x: 6,
                    y: 3,
                    char: '#',
                },
                Symbol {
                    x: 3,
                    y: 4,
                    char: '*',
                },
                Symbol {
                    x: 5,
                    y: 5,
                    char: '+',
                },
                Symbol {
                    x: 3,
                    y: 8,
                    char: '$',
                },
                Symbol {
                    x: 5,
                    y: 8,
                    char: '*',
                },
            ],
        );
        assert_eq!(part2(&input), 467835);
    }
}
