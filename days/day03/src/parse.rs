use std::{iter, ops::Range};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PartNumber {
    pub number: usize,
    pub x: Range<usize>,
    pub y: usize,
}

impl PartNumber {
    fn is_adjacent_to_symbol(&self, symbol: &Symbol) -> bool {
        (self.x.start.saturating_sub(1)..=self.x.end).contains(&symbol.x)
            && (self.y.saturating_sub(1)..=self.y + 1).contains(&symbol.y)
    }

    pub fn is_ajacent_to_any_symbol(&self, symbols: &[Symbol]) -> bool {
        symbols
            .iter()
            .any(|symbol| self.is_adjacent_to_symbol(symbol))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    pub x: usize,
    pub y: usize,
    pub char: char,
}

impl Symbol {
    fn ajacent_part_numbers(&self, part_numbers: &[PartNumber]) -> Vec<usize> {
        part_numbers
            .iter()
            .filter(|pn| pn.is_adjacent_to_symbol(self))
            .map(|pn| pn.number)
            .collect()
    }
    pub fn gear_ratio(&self, part_numbers: &[PartNumber]) -> Option<usize> {
        if self.char != '*' {
            return None;
        }
        let ajacent_parts = self.ajacent_part_numbers(part_numbers);
        if ajacent_parts.len() != 2{
            return None;
        }
        Some(ajacent_parts.iter().product())
    }
}

pub type StructuredInput = (Vec<PartNumber>, Vec<Symbol>);

pub fn parse(input: &str) -> StructuredInput {
    let mut part_numbers = vec![];
    let mut symbols = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut length: usize = 0;
        let mut number: Option<usize> = None;
        for (x, char) in line.chars().chain(iter::once('.')).enumerate() {
            if let Some(digit) = char.to_digit(10) {
                length += 1;
                // this essentially 'shifts' the number left if it already exists.
                number = number.map_or(Some(digit as usize), |n| Some(n * 10 + digit as usize))
            } else {
                if char != '.' {
                    symbols.push(Symbol { x, y, char })
                }
                // if number is not none, we must have just 'left' a number.
                if let Some(number) = number {
                    part_numbers.push(PartNumber {
                        number,
                        x: (x - length)..x,
                        y,
                    })
                }
                number = None;
                length = 0;
            }
        }
    }
    (part_numbers, symbols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gear_ratio() {
        let partnums = vec![
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
        ];
        let symbols = vec![
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
        ];
        assert_eq!(symbols[0].gear_ratio(&partnums), Some(467*35));
        assert_eq!(symbols[1].gear_ratio(&partnums), None);
    }

    #[test]
    fn test_num_ajacent_parts() {
        let partnums = vec![
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
        ];
        let symbols = vec![
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
        ];
        assert_eq!(symbols[0].ajacent_part_numbers(&partnums), [467, 35]);
        assert_eq!(symbols[1].ajacent_part_numbers(&partnums), [633]);
    }

    #[test]
    fn test_is_ajacent_to_symbol() {
        let partnums = vec![
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
        ];
        let symbols = vec![
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
        ];
        assert!(partnums[0].is_adjacent_to_symbol(&symbols[0]));
        assert!(!partnums[0].is_adjacent_to_symbol(&symbols[1]));
        assert!(partnums[3].is_adjacent_to_symbol(&symbols[1]));
    }

    #[test]
    fn test_is_ajacent_to_any_symbol() {
        let partnums = vec![
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
        ];
        let symbols = vec![
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
        ];
        assert!(partnums[0].is_ajacent_to_any_symbol(&symbols));
        assert!(!partnums[1].is_ajacent_to_any_symbol(&symbols));
        assert!(partnums[2].is_ajacent_to_any_symbol(&symbols));
        assert!(partnums[3].is_ajacent_to_any_symbol(&symbols));
        assert!(partnums[4].is_ajacent_to_any_symbol(&symbols));
        assert!(!partnums[5].is_ajacent_to_any_symbol(&symbols));
        assert!(partnums[6].is_ajacent_to_any_symbol(&symbols));
        assert!(partnums[7].is_ajacent_to_any_symbol(&symbols));
        assert!(partnums[8].is_ajacent_to_any_symbol(&symbols));
        assert!(partnums[9].is_ajacent_to_any_symbol(&symbols));
    }

    #[test]
    fn test_parse() {
        let input = concat!(
            "467..114..\n",
            "...*......\n",
            "..35..633.\n",
            "......#...\n",
            "617*......\n",
            ".....+.58.\n",
            "..592.....\n",
            "......755.\n",
            "...$.*....\n",
            ".664.598..\n",
        );
        assert_eq!(
            parse(input),
            (
                vec![
                    PartNumber {
                        number: 467,
                        x: 0..3,
                        y: 0
                    },
                    PartNumber {
                        number: 114,
                        x: 5..8,
                        y: 0
                    },
                    PartNumber {
                        number: 35,
                        x: 2..4,
                        y: 2
                    },
                    PartNumber {
                        number: 633,
                        x: 6..9,
                        y: 2
                    },
                    PartNumber {
                        number: 617,
                        x: 0..3,
                        y: 4
                    },
                    PartNumber {
                        number: 58,
                        x: 7..9,
                        y: 5
                    },
                    PartNumber {
                        number: 592,
                        x: 2..5,
                        y: 6
                    },
                    PartNumber {
                        number: 755,
                        x: 6..9,
                        y: 7
                    },
                    PartNumber {
                        number: 664,
                        x: 1..4,
                        y: 9
                    },
                    PartNumber {
                        number: 598,
                        x: 5..8,
                        y: 9
                    }
                ],
                vec![
                    Symbol {
                        x: 3,
                        y: 1,
                        char: '*'
                    },
                    Symbol {
                        x: 6,
                        y: 3,
                        char: '#'
                    },
                    Symbol {
                        x: 3,
                        y: 4,
                        char: '*'
                    },
                    Symbol {
                        x: 5,
                        y: 5,
                        char: '+'
                    },
                    Symbol {
                        x: 3,
                        y: 8,
                        char: '$'
                    },
                    Symbol {
                        x: 5,
                        y: 8,
                        char: '*'
                    }
                ]
            )
        );
    }
}
