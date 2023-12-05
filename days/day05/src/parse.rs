use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0},
    multi::{count, many1, separated_list1},
    sequence::{delimited, preceded, terminated},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    pub from: String,
    pub to: String,
    pub ranges: Vec<Range>,
}

impl Map {
    pub fn map(&self, src: u64)->u64 {
        for range in &self.ranges {
            if range.is_applicable(&src) {
                return range.map(src);
            }
        }
        src
    }
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (from, to)) = delimited(multispace0, Self::parse_to_from, multispace0)(input)?;
        let (input, ranges) = many1(Range::parse)(input)?;
        Ok((input, Map { from, to, ranges }))
    }
    fn parse_to_from(input: &str) -> IResult<&str, (String, String)> {
        let (input, from) = alpha1(input)?;
        let (input, to) = terminated(preceded(tag("-to-"), alpha1), tag(" map:\n"))(input)?;
        Ok((input, (from.to_string(), to.to_string())))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Range {
    pub dest_start: u64,
    pub src_start: u64,
    pub len: u64,
}

impl Range {
    fn is_applicable(&self, src: &u64) -> bool {
        self.src_start <= *src && *src < (self.src_start + self.len)
    }

    fn map(&self, src: u64) -> u64 {
        if self.is_applicable(&src) {
            let offset = src - self.src_start;
            self.dest_start + offset
        } else {
            src
        }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let number = delimited(multispace0, nom::character::complete::u64, multispace0);
        let (input, numbers) = count(number, 3)(input)?;
        Ok((
            input,
            Range {
                dest_start: numbers[0],
                src_start: numbers[1],
                len: numbers[2],
            },
        ))
    }
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<Map>)> {
    let (input, seeds) = preceded(
        tag("seeds: "),
        separated_list1(tag(" "), nom::character::complete::u64),
    )(input)?;
    let (input, maps) = many1(Map::parse)(input)?;
    Ok((input, (seeds, maps)))
}

pub fn parse(input: &str) -> (Vec<u64>, Vec<Map>) {
    parse_input(input).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let tested = Map {
            from: "seed".to_string(),
            to: "soil".to_string(),
            ranges: vec![
                Range {
                    dest_start: 50,
                    src_start: 98,
                    len: 2,
                },
                Range {
                    dest_start: 52,
                    src_start: 50,
                    len: 48,
                },
            ],
        };
        let input = [79, 14, 55, 13];
        let output = input.map(|i| tested.map(i));
        assert_eq!(output, [81, 14, 57, 13])
    }

    #[test]
    fn test_is_appliccable() {
        let tested = Range {
            dest_start: 50,
            src_start: 98,
            len: 2,
        };
        assert!(tested.is_applicable(&99));
        assert!(!tested.is_applicable(&100));
        assert!(!tested.is_applicable(&97));
    }
    #[test]
    fn test_range_map() {
        let tested = Range {
            dest_start: 52,
            src_start: 50,
            len: 48,
        };
        assert_eq!(tested.map(79), 81);
        assert_eq!(tested.map(100), 100);
    }

    #[test]
    fn test_map_parse() {
        let input = concat!("seed-to-soil map:\n", "50 98 2\n", "52 50 48\n", "\n",);
        assert_eq!(
            Map::parse(input).unwrap(),
            (
                "",
                Map {
                    from: "seed".to_string(),
                    to: "soil".to_string(),
                    ranges: vec![
                        Range {
                            dest_start: 50,
                            src_start: 98,
                            len: 2
                        },
                        Range {
                            dest_start: 52,
                            src_start: 50,
                            len: 48
                        },
                    ]
                },
            )
        )
    }

    #[test]
    fn test_range_parse() {
        assert_eq!(
            Range::parse("50 98 2\n").unwrap(),
            (
                "",
                Range {
                    dest_start: 50,
                    src_start: 98,
                    len: 2,
                }
            )
        );
        assert_eq!(
            Range::parse("0 15 37\n").unwrap(),
            (
                "",
                Range {
                    dest_start: 0,
                    src_start: 15,
                    len: 37,
                }
            )
        )
    }

    #[test]
    fn test_parse_to_from() {
        assert_eq!(
            Map::parse_to_from("seed-to-soil map:\n").unwrap(),
            ("", ("seed".to_string(), "soil".to_string()))
        );
        assert_eq!(
            Map::parse_to_from("hello-to-world map:\n").unwrap(),
            ("", ("hello".to_string(), "world".to_string()))
        );
    }

    #[test]
    fn test_parse_input() {
        let input = concat!(
            "seeds: 79 14 55 13\n",
            "\n",
            "seed-to-soil map:\n",
            "50 98 2\n",
            "52 50 48\n",
            "\n",
            "soil-to-fertilizer map:\n",
            "0 15 37\n",
            "37 52 2\n",
            "39 0 15\n",
        );
        assert_eq!(
            parse_input(input).unwrap(),
            (
                "",
                (
                    vec![79, 14, 55, 13],
                    vec![
                        Map {
                            from: "seed".to_string(),
                            to: "soil".to_string(),
                            ranges: vec![
                                Range {
                                    dest_start: 50,
                                    src_start: 98,
                                    len: 2
                                },
                                Range {
                                    dest_start: 52,
                                    src_start: 50,
                                    len: 48
                                },
                            ]
                        },
                        Map {
                            from: "soil".to_string(),
                            to: "fertilizer".to_string(),
                            ranges: vec![
                                Range {
                                    dest_start: 0,
                                    src_start: 15,
                                    len: 37
                                },
                                Range {
                                    dest_start: 37,
                                    src_start: 52,
                                    len: 2
                                },
                                Range {
                                    dest_start: 39,
                                    src_start: 0,
                                    len: 15
                                },
                            ]
                        }
                    ]
                )
            )
        );
    }

    #[test]
    fn test_parse() {
        let input = concat!(
            "seeds: 79 14 55 13\n",
            "\n",
            "seed-to-soil map:\n",
            "50 98 2\n",
            "52 50 48\n",
            "\n",
            "soil-to-fertilizer map:\n",
            "0 15 37\n",
            "37 52 2\n",
            "39 0 15\n",
            "\n",
            "fertilizer-to-water map:\n",
            "49 53 8\n",
            "0 11 42\n",
            "42 0 7\n",
            "57 7 4\n",
            "\n",
            "water-to-light map:\n",
            "88 18 7\n",
            "18 25 70\n",
            "\n",
            "light-to-temperature map:\n",
            "45 77 23\n",
            "81 45 19\n",
            "68 64 13\n",
            "\n",
            "temperature-to-humidity map:\n",
            "0 69 1\n",
            "1 0 69\n",
            "\n",
            "humidity-to-location map:\n",
            "60 56 37\n",
            "56 93 4\n",
        );
        assert_eq!(
            parse(input),
            (
                vec![79, 14, 55, 13],
                vec![
                    Map {
                        from: "seed".to_string(),
                        to: "soil".to_string(),
                        ranges: vec![
                            Range {
                                dest_start: 50,
                                src_start: 98,
                                len: 2
                            },
                            Range {
                                dest_start: 52,
                                src_start: 50,
                                len: 48
                            }
                        ]
                    },
                    Map {
                        from: "soil".to_string(),
                        to: "fertilizer".to_string(),
                        ranges: vec![
                            Range {
                                dest_start: 0,
                                src_start: 15,
                                len: 37
                            },
                            Range {
                                dest_start: 37,
                                src_start: 52,
                                len: 2
                            },
                            Range {
                                dest_start: 39,
                                src_start: 0,
                                len: 15
                            }
                        ]
                    },
                    Map {
                        from: "fertilizer".to_string(),
                        to: "water".to_string(),
                        ranges: vec![
                            Range {
                                dest_start: 49,
                                src_start: 53,
                                len: 8
                            },
                            Range {
                                dest_start: 0,
                                src_start: 11,
                                len: 42
                            },
                            Range {
                                dest_start: 42,
                                src_start: 0,
                                len: 7
                            },
                            Range {
                                dest_start: 57,
                                src_start: 7,
                                len: 4
                            }
                        ]
                    },
                    Map {
                        from: "water".to_string(),
                        to: "light".to_string(),
                        ranges: vec![
                            Range {
                                dest_start: 88,
                                src_start: 18,
                                len: 7
                            },
                            Range {
                                dest_start: 18,
                                src_start: 25,
                                len: 70
                            }
                        ]
                    },
                    Map {
                        from: "light".to_string(),
                        to: "temperature".to_string(),
                        ranges: vec![
                            Range {
                                dest_start: 45,
                                src_start: 77,
                                len: 23
                            },
                            Range {
                                dest_start: 81,
                                src_start: 45,
                                len: 19
                            },
                            Range {
                                dest_start: 68,
                                src_start: 64,
                                len: 13
                            }
                        ]
                    },
                    Map {
                        from: "temperature".to_string(),
                        to: "humidity".to_string(),
                        ranges: vec![
                            Range {
                                dest_start: 0,
                                src_start: 69,
                                len: 1
                            },
                            Range {
                                dest_start: 1,
                                src_start: 0,
                                len: 69
                            }
                        ]
                    },
                    Map {
                        from: "humidity".to_string(),
                        to: "location".to_string(),
                        ranges: vec![
                            Range {
                                dest_start: 60,
                                src_start: 56,
                                len: 37
                            },
                            Range {
                                dest_start: 56,
                                src_start: 93,
                                len: 4
                            }
                        ]
                    }
                ]
            )
        );
    }
}
