use crate::parse::*;

pub fn part2(input: &[Game]) -> usize {
    input.iter().map(|g| g.min_contents().power()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
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
        assert_eq!(part2(&input), 2286);
    }
}
