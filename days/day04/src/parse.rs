use std::collections::BTreeSet;

use nom::{
    bytes::complete::tag,
    multi::{many1, separated_list1},
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Debug, PartialEq, Eq,)]
pub struct Card {
    pub id: usize,
    pub winning_numbers: BTreeSet<u8>,
    pub numbers: BTreeSet<u8>,
}

impl Card {
    pub fn parse(input: &str) -> IResult<&str, Card> {
        let mut game_id = terminated(
            preceded(
                preceded(tag("Card"), many1(tag(" "))),
                nom::character::complete::u8,
            ),
            preceded(tag(":"), many1(tag(" "))),
        );
        let mut number_list = separated_list1(many1(tag(" ")), nom::character::complete::u8);
        let seperator = preceded(tag(" |"), many1(tag(" ")));
        let (input, id) = game_id(input)?;
        let (input, winning_numbers) = number_list(input)?;
        let (input, numbers) = preceded(seperator, number_list)(input)?;
        let winning_numbers: BTreeSet<u8> = winning_numbers.into_iter().collect();
        let numbers: BTreeSet<u8> = numbers.into_iter().collect();
        Ok((
            input,
            Card {
                id: id as usize,
                winning_numbers,
                numbers,
            },
        ))
    }

    pub fn winning_number_matches(&self) -> BTreeSet<u8> {
        self.numbers
            .intersection(&self.winning_numbers)
            .cloned()
            .collect()
    }
}

pub fn parse(input: &str) -> Vec<Card> {
    input.lines().map(|l| Card::parse(l).unwrap().1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use collection_literals::collection;

    #[test]
    fn test_parse() {
        let input = concat!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n",
        );
        assert_eq!(
            parse(input),
            vec![
                Card {
                    id: 1,
                    winning_numbers: collection! {17, 41, 48, 83, 86},
                    numbers: collection! {6, 9, 17, 31, 48, 53, 83, 86}
                },
                Card {
                    id: 2,
                    winning_numbers: collection! {13, 16, 20, 32, 61},
                    numbers: collection! {17, 19, 24, 30, 32, 61, 68, 82}
                },
                Card {
                    id: 3,
                    winning_numbers: collection! {1, 21, 44, 53, 59},
                    numbers: collection! {1, 14, 16, 21, 63, 69, 72, 82}
                },
                Card {
                    id: 4,
                    winning_numbers: collection! {41, 69, 73, 84, 92},
                    numbers: collection! {5, 51, 54, 58, 59, 76, 83, 84}
                },
                Card {
                    id: 5,
                    winning_numbers: collection! {26, 28, 32, 83, 87},
                    numbers: collection! {12, 22, 30, 36, 70, 82, 88, 93}
                },
                Card {
                    id: 6,
                    winning_numbers: collection! {13 , 18, 31, 56, 72},
                    numbers: collection! {10, 11, 23, 35, 36, 67, 74, 77}
                }
            ]
        );
    }

    #[test]
    fn test_card_parse() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(
            Card::parse(input).unwrap(),
            (
                "",
                Card {
                    id: 1,
                    winning_numbers: collection! {41, 48, 83, 86, 17},
                    numbers: collection! {83, 86, 6, 31, 17, 9, 48, 53},
                }
            )
        )
    }
}
