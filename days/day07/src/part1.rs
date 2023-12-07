use crate::parse::*;

pub fn part1(input: &mut [(Hand, u32)]) -> usize {
    input.sort_by(|a, b| a.0.cmp(&b.0) );
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
    fn test_part1() {
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
        assert_eq!(part1(&mut input), 6592);
    }
}
