pub fn part2(input: &[Vec<char>]) -> usize {
    input
        .iter()
        .map(|s| {
            let mut string = String::new();
            string.push_str(&s.first().unwrap().to_string());
            string.push_str(&s.last().unwrap().to_string());
            string.parse::<usize>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = vec![
            vec!['2', '1', '9'],
            vec!['8', '2', '3'],
            vec!['1', '2', '3'],
            vec!['2', '1', '3', '4'],
            vec!['4', '9', '8', '7', '2'],
            vec!['1', '8', '2', '3', '4'],
            vec!['7', '6'],
        ];
        assert_eq!(part2(&input), 281);
    }
}
