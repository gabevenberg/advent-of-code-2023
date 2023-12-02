use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{self},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Handful {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Handful {
    pub fn is_possible_given_contents(&self, contents: &Handful) -> bool {
        contents.red >= self.red && contents.green >= self.green && contents.blue >= self.blue
    }

    pub fn power(&self) -> usize {
        self.red as usize * self.green as usize * self.blue as usize
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    pub id: u8,
    pub handfuls: Vec<Handful>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, id) = Self::parse_game_id(input)?;
        let (input, handfuls) = separated_list1(tag("; "), Self::parse_handful)(input)?;
        Ok((input, Game { id, handfuls }))
    }

    fn parse_game_id(input: &str) -> IResult<&str, u8> {
        terminated(
            preceded(tag("Game "), nom::character::complete::u8),
            tag(": "),
        )(input)
    }

    fn parse_colour_set(input: &str) -> IResult<&str, Colour> {
        let red = terminated(character::complete::u8, tag(" red"));
        let green = terminated(character::complete::u8, tag(" green"));
        let blue = terminated(character::complete::u8, tag(" blue"));
        alt((
            map(red, Colour::Red),
            map(green, Colour::Green),
            map(blue, Colour::Blue),
        ))(input)
    }

    fn parse_handful(input: &str) -> IResult<&str, Handful> {
        let (input, colours) = separated_list1(tag(", "), Self::parse_colour_set)(input)?;
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for colour in colours {
            match colour {
                Colour::Red(n) => red = n,
                Colour::Green(n) => green = n,
                Colour::Blue(n) => blue = n,
            }
        }
        Ok((input, Handful { red, green, blue }))
    }

    pub fn is_possible_given_contents(&self, contents: &Handful) -> bool {
        self.handfuls
            .iter()
            .all(|h| h.is_possible_given_contents(contents))
    }

    pub fn min_contents(&self) -> Handful {
        Handful {
            red: self.handfuls.iter().map(|g| g.red).max().unwrap_or(0),
            green: self.handfuls.iter().map(|g| g.green).max().unwrap_or(0),
            blue: self.handfuls.iter().map(|g| g.blue).max().unwrap_or(0),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Colour {
    Red(u8),
    Green(u8),
    Blue(u8),
}

pub fn parse(input: &str) -> Vec<Game> {
    input.lines().map(|l| Game::parse(l).unwrap().1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = concat!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n",
        );
        assert_eq!(
            parse(input),
            vec![
                Game {
                    id: 1,
                    handfuls: vec![
                        Handful {
                            red: 4,
                            green: 0,
                            blue: 3
                        },
                        Handful {
                            red: 1,
                            green: 2,
                            blue: 6
                        },
                        Handful {
                            red: 0,
                            green: 2,
                            blue: 0
                        }
                    ]
                },
                Game {
                    id: 2,
                    handfuls: vec![
                        Handful {
                            red: 0,
                            green: 2,
                            blue: 1
                        },
                        Handful {
                            red: 1,
                            green: 3,
                            blue: 4
                        },
                        Handful {
                            red: 0,
                            green: 1,
                            blue: 1
                        }
                    ]
                },
                Game {
                    id: 3,
                    handfuls: vec![
                        Handful {
                            red: 20,
                            green: 8,
                            blue: 6
                        },
                        Handful {
                            red: 4,
                            green: 13,
                            blue: 5
                        },
                        Handful {
                            red: 1,
                            green: 5,
                            blue: 0
                        }
                    ]
                },
                Game {
                    id: 4,
                    handfuls: vec![
                        Handful {
                            red: 3,
                            green: 1,
                            blue: 6
                        },
                        Handful {
                            red: 6,
                            green: 3,
                            blue: 0
                        },
                        Handful {
                            red: 14,
                            green: 3,
                            blue: 15
                        }
                    ]
                },
                Game {
                    id: 5,
                    handfuls: vec![
                        Handful {
                            red: 6,
                            green: 3,
                            blue: 1
                        },
                        Handful {
                            red: 1,
                            green: 2,
                            blue: 2
                        }
                    ]
                }
            ]
        );
    }

    #[test]
    fn test_game_parsing() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(
            Game::parse(input).unwrap(),
            (
                "",
                Game {
                    id: 1,
                    handfuls: vec![
                        Handful {
                            blue: 3,
                            red: 4,
                            green: 0
                        },
                        Handful {
                            red: 1,
                            green: 2,
                            blue: 6
                        },
                        Handful {
                            green: 2,
                            blue: 0,
                            red: 0
                        },
                    ]
                }
            )
        )
    }

    #[test]
    fn test_parse_handful() {
        assert_eq!(
            Game::parse_handful("10 blue, 53 red, 3 green").unwrap(),
            (
                "",
                Handful {
                    red: 53,
                    green: 3,
                    blue: 10
                }
            )
        );
        assert_eq!(
            Game::parse_handful("15 green, 53 red").unwrap(),
            (
                "",
                Handful {
                    red: 53,
                    green: 15,
                    blue: 0
                }
            )
        );
    }

    #[test]
    fn test_parse_colour_set() {
        assert_eq!(
            Game::parse_colour_set("5 red").unwrap(),
            ("", Colour::Red(5))
        );
        assert_eq!(
            Game::parse_colour_set("10 blue").unwrap(),
            ("", Colour::Blue(10))
        );
        assert_eq!(
            Game::parse_colour_set("123 green").unwrap(),
            ("", Colour::Green(123))
        );
    }

    #[test]
    fn test_parse_game_id() {
        assert_eq!(Game::parse_game_id("Game 2: ").unwrap(), ("", 2));
        assert_eq!(Game::parse_game_id("Game 100: ").unwrap(), ("", 100));
        assert_eq!(Game::parse_game_id("Game 15: ").unwrap(), ("", 15));
    }
}
