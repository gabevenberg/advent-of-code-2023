use crate::parse::*;

pub fn part2(input: &[(Hand, u32)]) -> usize {
    let mut input: Vec<(Hand, u32)> = input
        .iter()
        .map(|c| (c.0.turn_jacks_to_jokers(), c.1))
        .collect();
    input.sort_by_key(|set| set.0);
    input
        .iter()
        .enumerate()
        .map(|i| (i.0 + 1) * i.1 .1 as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = vec![
            (
                Hand {
                    cards: [Card::Two, Card::Three, Card::Four, Card::Five, Card::Ace],
                },
                1,
            ),
            (
                Hand {
                    cards: [Card::Queen, Card::Two, Card::King, Card::Joker, Card::Joker],
                },
                13,
            ),
            (
                Hand {
                    cards: [Card::Queen, Card::Two, Card::Queen, Card::Two, Card::Queen],
                },
                19,
            ),
            (
                Hand {
                    cards: [Card::Tim, Card::Three, Card::Tim, Card::Three, Card::Joker],
                },
                17,
            ),
            (
                Hand {
                    cards: [
                        Card::Tim,
                        Card::Three,
                        Card::Queen,
                        Card::Three,
                        Card::Three,
                    ],
                },
                11,
            ),
            (
                Hand {
                    cards: [Card::Two, Card::Three, Card::Four, Card::Five, Card::Joker],
                },
                3,
            ),
            (
                Hand {
                    cards: [Card::Joker, Card::Three, Card::Four, Card::Five, Card::Ace],
                },
                2,
            ),
            (
                Hand {
                    cards: [Card::Three, Card::Two, Card::Tim, Card::Three, Card::King],
                },
                5,
            ),
            (
                Hand {
                    cards: [Card::Tim, Card::Five, Card::Five, Card::Joker, Card::Five],
                },
                29,
            ),
            (
                Hand {
                    cards: [Card::King, Card::King, Card::Six, Card::Seven, Card::Seven],
                },
                7,
            ),
            (
                Hand {
                    cards: [Card::King, Card::Tim, Card::Joker, Card::Joker, Card::Tim],
                },
                34,
            ),
            (
                Hand {
                    cards: [
                        Card::Queen,
                        Card::Queen,
                        Card::Queen,
                        Card::Joker,
                        Card::Ace,
                    ],
                },
                31,
            ),
            (
                Hand {
                    cards: [
                        Card::Joker,
                        Card::Joker,
                        Card::Joker,
                        Card::Joker,
                        Card::Joker,
                    ],
                },
                37,
            ),
            (
                Hand {
                    cards: [Card::Joker, Card::Ace, Card::Ace, Card::Ace, Card::Ace],
                },
                43,
            ),
            (
                Hand {
                    cards: [Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Joker],
                },
                59,
            ),
            (
                Hand {
                    cards: [Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace],
                },
                61,
            ),
            (
                Hand {
                    cards: [Card::Two, Card::Ace, Card::Ace, Card::Ace, Card::Ace],
                },
                23,
            ),
            (
                Hand {
                    cards: [
                        Card::Two,
                        Card::Joker,
                        Card::Joker,
                        Card::Joker,
                        Card::Joker,
                    ],
                },
                53,
            ),
            (
                Hand {
                    cards: [
                        Card::Joker,
                        Card::Joker,
                        Card::Joker,
                        Card::Joker,
                        Card::Two,
                    ],
                },
                41,
            ),
        ];
        assert_eq!(part2(&input), 6839);
    }
}
