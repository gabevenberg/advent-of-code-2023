use crate::parse::*;

pub fn part2(input: &[Card]) -> usize {
    let mut stack: Vec<Card> = input.to_vec();
    for i in 0..stack.len() {
        let matches = stack[i].num_matches();
        if matches >= 1 {
            //+1 because we want 3 matches to give the num 1,2,3
            for advance in 1..(matches+1){
                stack[i+advance].multiplier+=stack[i].multiplier
            }
        }
    }
    stack.iter().map(|c| c.multiplier).sum()
}

#[cfg(test)]
mod tests {
    use collection_literals::collection;

    use super::*;

    #[test]
    fn test_part2() {
        let input = vec![
            Card {
                id: 1,
                winning_numbers: collection! {17, 41, 48, 83, 86},
                numbers: collection! {6, 9, 17, 31, 48, 53, 83, 86},
                multiplier: 1,
            },
            Card {
                id: 2,
                winning_numbers: collection! {13, 16, 20, 32, 61},
                numbers: collection! {17, 19, 24, 30, 32, 61, 68, 82},
                multiplier: 1,
            },
            Card {
                id: 3,
                winning_numbers: collection! {1, 21, 44, 53, 59},
                numbers: collection! {1, 14, 16, 21, 63, 69, 72, 82},
                multiplier: 1,
            },
            Card {
                id: 4,
                winning_numbers: collection! {41, 69, 73, 84, 92},
                numbers: collection! {5, 51, 54, 58, 59, 76, 83, 84},
                multiplier: 1,
            },
            Card {
                id: 5,
                winning_numbers: collection! {26, 28, 32, 83, 87},
                numbers: collection! {12, 22, 30, 36, 70, 82, 88, 93},
                multiplier: 1,
            },
            Card {
                id: 6,
                winning_numbers: collection! {13 , 18, 31, 56, 72},
                numbers: collection! {10, 11, 23, 35, 36, 67, 74, 77},
                multiplier: 1,
            },
        ];
        assert_eq!(part2(&input), 30);
    }
}
