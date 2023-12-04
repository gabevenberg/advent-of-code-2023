use crate::parse::*;

pub fn part1(input: &[Card]) -> usize {
    input
        .iter()
        .map(|c| {
            let len = c.winning_number_matches().len();
            if len >=1{
                2_usize.pow((len-1).try_into().unwrap())
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use collection_literals::collection;

    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            Card {
                id: 1,
                winning_numbers: collection! {17, 41, 48, 83, 86},
                numbers: collection! {6, 9, 17, 31, 48, 53, 83, 86},
            },
            Card {
                id: 2,
                winning_numbers: collection! {13, 16, 20, 32, 61},
                numbers: collection! {17, 19, 24, 30, 32, 61, 68, 82},
            },
            Card {
                id: 3,
                winning_numbers: collection! {1, 21, 44, 53, 59},
                numbers: collection! {1, 14, 16, 21, 63, 69, 72, 82},
            },
            Card {
                id: 4,
                winning_numbers: collection! {41, 69, 73, 84, 92},
                numbers: collection! {5, 51, 54, 58, 59, 76, 83, 84},
            },
            Card {
                id: 5,
                winning_numbers: collection! {26, 28, 32, 83, 87},
                numbers: collection! {12, 22, 30, 36, 70, 82, 88, 93},
            },
            Card {
                id: 6,
                winning_numbers: collection! {13 , 18, 31, 56, 72},
                numbers: collection! {10, 11, 23, 35, 36, 67, 74, 77},
            },
        ];
        assert_eq!(part1(&input), 13);
    }
}
