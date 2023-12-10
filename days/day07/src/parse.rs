use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Tim = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Formation {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    formation: Formation,
    cards: [Card; 5],
}

impl Hand {
    pub fn new(cards: [Card; 5]) -> Hand {
        Hand {
            formation: Self::determine_formation(&cards),
            cards,
        }
    }
    fn determine_formation(cards: &[Card; 5]) -> Formation {
        let mut counts = cards.iter().counts();
        let jokers = counts.remove(&Card::Joker).unwrap_or(0);
        let types = counts.len();
        let max_of_type = counts.into_values().max().unwrap_or(0) + jokers;
        match types {
            // if 0 types, they are all joker.
            0 => Formation::FiveOfKind,
            1 => Formation::FiveOfKind,
            2 => {
                // if there are 4 of 1 kind, doesnt matter what the other card is. If there are
                // three of a kind but only 2 card types, the other 2 must be a pair.
                match max_of_type {
                    4 => Formation::FourOfKind,
                    3 => Formation::FullHouse,
                    _ => panic!("idk what type of hand this is: {:?}", cards),
                }
            }
            3 => {
                // if there are 3 types, it could either be three of a kind, or two pair.
                match max_of_type {
                    3 => Formation::ThreeOfKind,
                    2 => Formation::TwoPair,
                    _ => panic!("idk what type of hand this is: {:?}", cards),
                }
            }
            4 => Formation::OnePair,
            5 => Formation::HighCard,
            _ => panic!("how are there more than 5 types!"),
        }
    }
    pub fn turn_jacks_to_jokers(&mut self) {
        self.cards = self
            .cards
            .map(|c| if c == Card::Jack { Card::Joker } else { c });
    }
}

pub fn parse(input: &str) -> Vec<(Hand, u32)> {
    input
        .lines()
        .map(|line| {
            let mut splits = line.split(' ');
            let hand = splits.next().unwrap();
            let bid: u32 = splits.next().unwrap().parse().unwrap();
            let hand: Vec<Card> = hand
                .chars()
                .map(|c| match c {
                    'A' => Card::Ace,
                    'K' => Card::King,
                    'Q' => Card::Queen,
                    'J' => Card::Jack,
                    'T' => Card::Tim,
                    '9' => Card::Nine,
                    '8' => Card::Eight,
                    '7' => Card::Seven,
                    '6' => Card::Six,
                    '5' => Card::Five,
                    '4' => Card::Four,
                    '3' => Card::Three,
                    '2' => Card::Two,
                    e => panic!("invalid card {}", e),
                })
                .collect();
            (Hand::new(hand.try_into().unwrap()), bid)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ord_with_joker_conversion() {
        let mut input = vec![
            (
                Hand::new([Card::Two, Card::Three, Card::Four, Card::Five, Card::Ace]),
                1,
            ),
            (
                Hand::new([Card::Queen, Card::Two, Card::King, Card::Joker, Card::Joker]),
                13,
            ),
            (
                Hand::new([Card::Queen, Card::Two, Card::Queen, Card::Two, Card::Queen]),
                19,
            ),
            (
                Hand::new([Card::Tim, Card::Three, Card::Tim, Card::Three, Card::Joker]),
                17,
            ),
            (
                Hand::new([
                    Card::Tim,
                    Card::Three,
                    Card::Queen,
                    Card::Three,
                    Card::Three,
                ]),
                11,
            ),
            (
                Hand::new([Card::Two, Card::Three, Card::Four, Card::Five, Card::Joker]),
                3,
            ),
            (
                Hand::new([Card::Joker, Card::Three, Card::Four, Card::Five, Card::Ace]),
                2,
            ),
            (
                Hand::new([Card::Three, Card::Two, Card::Tim, Card::Three, Card::King]),
                5,
            ),
            (
                Hand::new([Card::Tim, Card::Five, Card::Five, Card::Joker, Card::Five]),
                29,
            ),
            (
                Hand::new([Card::King, Card::King, Card::Six, Card::Seven, Card::Seven]),
                7,
            ),
            (
                Hand::new([Card::King, Card::Tim, Card::Joker, Card::Joker, Card::Tim]),
                34,
            ),
            (
                Hand::new([
                    Card::Queen,
                    Card::Queen,
                    Card::Queen,
                    Card::Joker,
                    Card::Ace,
                ]),
                31,
            ),
            (
                Hand::new([
                    Card::Joker,
                    Card::Joker,
                    Card::Joker,
                    Card::Joker,
                    Card::Joker,
                ]),
                37,
            ),
            (
                Hand::new([Card::Joker, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
                43,
            ),
            (
                Hand::new([Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Joker]),
                59,
            ),
            (
                Hand::new([Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
                61,
            ),
            (
                Hand::new([Card::Two, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
                23,
            ),
            (
                Hand::new([
                    Card::Two,
                    Card::Joker,
                    Card::Joker,
                    Card::Joker,
                    Card::Joker,
                ]),
                53,
            ),
            (
                Hand::new([
                    Card::Joker,
                    Card::Joker,
                    Card::Joker,
                    Card::Joker,
                    Card::Two,
                ]),
                41,
            ),
        ];
        input.iter_mut().for_each(|i| i.0.turn_jacks_to_jokers());
        input.sort_by(|a, b| a.0.cmp(&b.0));
        println!("{:#?}", input);
        //check that the bids are sorted (the input is curated to ensure that the bids are sorted
        //when the hands are sorted.)
        assert!(input.windows(2).all(|w| w[0].1 <= w[1].1))
    }

    #[test]
    fn test_determine_formation_after_joker_conversion() {
        let mut tests = vec![
            (
                Hand::new([Card::Two, Card::Three, Card::Four, Card::Five, Card::Ace]),
                Formation::HighCard,
            ),
            (
                Hand::new([Card::Queen, Card::Two, Card::King, Card::Joker, Card::Joker]),
                Formation::ThreeOfKind,
            ),
            (
                Hand::new([Card::Queen, Card::Two, Card::Queen, Card::Two, Card::Queen]),
                Formation::FullHouse,
            ),
            (
                Hand::new([Card::Tim, Card::Three, Card::Tim, Card::Three, Card::Joker]),
                Formation::FullHouse,
            ),
            (
                Hand::new([
                    Card::Tim,
                    Card::Three,
                    Card::Queen,
                    Card::Three,
                    Card::Three,
                ]),
                Formation::ThreeOfKind,
            ),
            (
                Hand::new([Card::Two, Card::Three, Card::Four, Card::Five, Card::Joker]),
                Formation::OnePair,
            ),
            (
                Hand::new([Card::Joker, Card::Three, Card::Four, Card::Five, Card::Ace]),
                Formation::OnePair,
            ),
            (
                Hand::new([Card::Three, Card::Two, Card::Tim, Card::Three, Card::King]),
                Formation::OnePair,
            ),
            (
                Hand::new([Card::Tim, Card::Five, Card::Five, Card::Joker, Card::Five]),
                Formation::FourOfKind,
            ),
            (
                Hand::new([Card::King, Card::King, Card::Six, Card::Seven, Card::Seven]),
                Formation::TwoPair,
            ),
            (
                Hand::new([Card::King, Card::Tim, Card::Joker, Card::Joker, Card::Tim]),
                Formation::FourOfKind,
            ),
            (
                Hand::new([
                    Card::Queen,
                    Card::Queen,
                    Card::Queen,
                    Card::Joker,
                    Card::Ace,
                ]),
                Formation::FourOfKind,
            ),
            (
                Hand::new([
                    Card::Joker,
                    Card::Joker,
                    Card::Joker,
                    Card::Joker,
                    Card::Joker,
                ]),
                Formation::FiveOfKind,
            ),
            (
                Hand::new([Card::Joker, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
                Formation::FiveOfKind,
            ),
            (
                Hand::new([Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Joker]),
                Formation::FiveOfKind,
            ),
            (
                Hand::new([Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
                Formation::FiveOfKind,
            ),
            (
                Hand::new([Card::Two, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
                Formation::FourOfKind,
            ),
            (
                Hand::new([
                    Card::Two,
                    Card::Joker,
                    Card::Joker,
                    Card::Joker,
                    Card::Joker,
                ]),
                Formation::FiveOfKind,
            ),
            (
                Hand::new([
                    Card::Joker,
                    Card::Joker,
                    Card::Joker,
                    Card::Joker,
                    Card::Two,
                ]),
                Formation::FiveOfKind,
            ),
        ];
        tests.iter_mut().for_each(|i| i.0.turn_jacks_to_jokers());
        for test in tests {
            assert_eq!(test.0.formation, test.1)
        }
    }

    #[test]
    fn test_ord() {
        let mut input = vec![
            (
                Hand::new([Card::Two, Card::Three, Card::Four, Card::Five, Card::Ace]),
                1,
            ),
            (
                Hand::new([Card::Queen, Card::Two, Card::King, Card::Jack, Card::Jack]),
                13,
            ),
            (
                Hand::new([Card::Queen, Card::Two, Card::Queen, Card::Two, Card::Queen]),
                19,
            ),
            (
                Hand::new([Card::Tim, Card::Three, Card::Tim, Card::Three, Card::Jack]),
                17,
            ),
            (
                Hand::new([
                    Card::Tim,
                    Card::Three,
                    Card::Queen,
                    Card::Three,
                    Card::Three,
                ]),
                11,
            ),
            (
                Hand::new([Card::Two, Card::Three, Card::Four, Card::Five, Card::Jack]),
                3,
            ),
            (
                Hand::new([Card::Jack, Card::Three, Card::Four, Card::Five, Card::Ace]),
                2,
            ),
            (
                Hand::new([Card::Three, Card::Two, Card::Tim, Card::Three, Card::King]),
                5,
            ),
            (
                Hand::new([Card::Tim, Card::Five, Card::Five, Card::Jack, Card::Five]),
                29,
            ),
            (
                Hand::new([Card::King, Card::King, Card::Six, Card::Seven, Card::Seven]),
                7,
            ),
            (
                Hand::new([Card::King, Card::Tim, Card::Jack, Card::Jack, Card::Tim]),
                34,
            ),
            (
                Hand::new([Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace]),
                31,
            ),
            (
                Hand::new([Card::Jack, Card::Jack, Card::Jack, Card::Jack, Card::Jack]),
                37,
            ),
            (
                Hand::new([Card::Jack, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
                43,
            ),
            (
                Hand::new([Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Jack]),
                59,
            ),
            (
                Hand::new([Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
                61,
            ),
            (
                Hand::new([Card::Two, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
                23,
            ),
            (
                Hand::new([Card::Two, Card::Jack, Card::Jack, Card::Jack, Card::Jack]),
                53,
            ),
            (
                Hand::new([Card::Jack, Card::Jack, Card::Jack, Card::Jack, Card::Two]),
                41,
            ),
        ];
        input.sort_by(|a, b| a.0.cmp(&b.0));
        println!("{:#?}", input);
        assert_eq!(
            input.iter().map(|h| h.1).collect::<Vec<i32>>(),
            vec![3, 1, 2, 5, 13, 17, 34, 7, 11, 29, 31, 19, 53, 23, 41, 43, 59, 37, 61]
        )
    }

    #[test]
    fn test_determine_formation() {
        let tests = vec![
            (
                Hand::new([Card::Two, Card::Three, Card::Four, Card::Five, Card::Ace]),
                Formation::HighCard,
            ),
            (
                Hand::new([Card::Queen, Card::Two, Card::King, Card::Jack, Card::Jack]),
                Formation::OnePair,
            ),
            (
                Hand::new([Card::Queen, Card::Two, Card::Queen, Card::Two, Card::Queen]),
                Formation::FullHouse,
            ),
            (
                Hand::new([Card::Tim, Card::Three, Card::Tim, Card::Three, Card::Jack]),
                Formation::TwoPair,
            ),
            (
                Hand::new([
                    Card::Tim,
                    Card::Three,
                    Card::Queen,
                    Card::Three,
                    Card::Three,
                ]),
                Formation::ThreeOfKind,
            ),
            (
                Hand::new([Card::Two, Card::Three, Card::Four, Card::Five, Card::Jack]),
                Formation::HighCard,
            ),
            (
                Hand::new([Card::Jack, Card::Three, Card::Four, Card::Five, Card::Ace]),
                Formation::HighCard,
            ),
            (
                Hand::new([Card::Three, Card::Two, Card::Tim, Card::Three, Card::King]),
                Formation::OnePair,
            ),
            (
                Hand::new([Card::Tim, Card::Five, Card::Five, Card::Jack, Card::Five]),
                Formation::ThreeOfKind,
            ),
            (
                Hand::new([Card::King, Card::King, Card::Six, Card::Seven, Card::Seven]),
                Formation::TwoPair,
            ),
            (
                Hand::new([Card::King, Card::Tim, Card::Jack, Card::Jack, Card::Tim]),
                Formation::TwoPair,
            ),
            (
                Hand::new([Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace]),
                Formation::ThreeOfKind,
            ),
            (
                Hand::new([Card::Jack, Card::Jack, Card::Jack, Card::Jack, Card::Jack]),
                Formation::FiveOfKind,
            ),
            (
                Hand::new([Card::Jack, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
                Formation::FourOfKind,
            ),
            (
                Hand::new([Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Jack]),
                Formation::FourOfKind,
            ),
            (
                Hand::new([Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
                Formation::FiveOfKind,
            ),
            (
                Hand::new([Card::Two, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
                Formation::FourOfKind,
            ),
            (
                Hand::new([Card::Two, Card::Jack, Card::Jack, Card::Jack, Card::Jack]),
                Formation::FourOfKind,
            ),
            (
                Hand::new([Card::Jack, Card::Jack, Card::Jack, Card::Jack, Card::Two]),
                Formation::FourOfKind,
            ),
        ];
        for test in tests {
            assert_eq!(test.0.formation, test.1)
        }
    }

    #[test]
    fn test_parse() {
        let input = concat!(
            "2345A 1\n",
            "Q2KJJ 13\n",
            "Q2Q2Q 19\n",
            "T3T3J 17\n",
            "T3Q33 11\n",
            "2345J 3\n",
            "J345A 2\n",
            "32T3K 5\n",
            "T55J5 29\n",
            "KK677 7\n",
            "KTJJT 34\n",
            "QQQJA 31\n",
            "JJJJJ 37\n",
            "JAAAA 43\n",
            "AAAAJ 59\n",
            "AAAAA 61\n",
            "2AAAA 23\n",
            "2JJJJ 53\n",
            "JJJJ2 41\n",
        );
        assert_eq!(
            parse(input),
            vec![
                (
                    Hand::new([Card::Two, Card::Three, Card::Four, Card::Five, Card::Ace]),
                    1
                ),
                (
                    Hand::new([Card::Queen, Card::Two, Card::King, Card::Jack, Card::Jack]),
                    13
                ),
                (
                    Hand::new([Card::Queen, Card::Two, Card::Queen, Card::Two, Card::Queen]),
                    19
                ),
                (
                    Hand::new([Card::Tim, Card::Three, Card::Tim, Card::Three, Card::Jack]),
                    17
                ),
                (
                    Hand::new([
                        Card::Tim,
                        Card::Three,
                        Card::Queen,
                        Card::Three,
                        Card::Three
                    ]),
                    11
                ),
                (
                    Hand::new([Card::Two, Card::Three, Card::Four, Card::Five, Card::Jack]),
                    3
                ),
                (
                    Hand::new([Card::Jack, Card::Three, Card::Four, Card::Five, Card::Ace]),
                    2
                ),
                (
                    Hand::new([Card::Three, Card::Two, Card::Tim, Card::Three, Card::King]),
                    5
                ),
                (
                    Hand::new([Card::Tim, Card::Five, Card::Five, Card::Jack, Card::Five]),
                    29
                ),
                (
                    Hand::new([Card::King, Card::King, Card::Six, Card::Seven, Card::Seven]),
                    7
                ),
                (
                    Hand::new([Card::King, Card::Tim, Card::Jack, Card::Jack, Card::Tim]),
                    34
                ),
                (
                    Hand::new([Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace]),
                    31
                ),
                (
                    Hand::new([Card::Jack, Card::Jack, Card::Jack, Card::Jack, Card::Jack]),
                    37
                ),
                (
                    Hand::new([Card::Jack, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
                    43
                ),
                (
                    Hand::new([Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Jack]),
                    59
                ),
                (
                    Hand::new([Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
                    61
                ),
                (
                    Hand::new([Card::Two, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
                    23
                ),
                (
                    Hand::new([Card::Two, Card::Jack, Card::Jack, Card::Jack, Card::Jack]),
                    53
                ),
                (
                    Hand::new([Card::Jack, Card::Jack, Card::Jack, Card::Jack, Card::Two]),
                    41
                )
            ]
        );
    }
}
