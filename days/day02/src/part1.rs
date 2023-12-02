use crate::parse::*;

pub fn part1(input: &[Game]) -> usize {
    let contents = Handful {
        red: 12,
        green: 13,
        blue: 14,
    };
    input
        .iter()
        .filter(|g| g.is_possible_given_contents(&contents))
        .map(|g| g.id as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            Game {
                id: 1,
                handfuls: vec![
                    Handful {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Handful {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Handful {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ],
            },
            Game {
                id: 2,
                handfuls: vec![
                    Handful {
                        red: 0,
                        green: 2,
                        blue: 1,
                    },
                    Handful {
                        red: 1,
                        green: 3,
                        blue: 4,
                    },
                    Handful {
                        red: 0,
                        green: 1,
                        blue: 1,
                    },
                ],
            },
            Game {
                id: 3,
                handfuls: vec![
                    Handful {
                        red: 20,
                        green: 8,
                        blue: 6,
                    },
                    Handful {
                        red: 4,
                        green: 13,
                        blue: 5,
                    },
                    Handful {
                        red: 1,
                        green: 5,
                        blue: 0,
                    },
                ],
            },
            Game {
                id: 4,
                handfuls: vec![
                    Handful {
                        red: 3,
                        green: 1,
                        blue: 6,
                    },
                    Handful {
                        red: 6,
                        green: 3,
                        blue: 0,
                    },
                    Handful {
                        red: 14,
                        green: 3,
                        blue: 15,
                    },
                ],
            },
            Game {
                id: 5,
                handfuls: vec![
                    Handful {
                        red: 6,
                        green: 3,
                        blue: 1,
                    },
                    Handful {
                        red: 1,
                        green: 2,
                        blue: 2,
                    },
                ],
            },
        ];
        assert_eq!(part1(&input), 8);
    }
}
