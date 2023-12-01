pub fn part1(input: &[Vec<char>]) -> usize {
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
    fn test_part1() {
        let input = vec![
            vec!['1', '2'],
            vec!['3', '8'],
            vec!['1', '2', '3', '4', '5'],
            vec!['7'],
        ];
        assert_eq!(part1(&input), 142);
    }
}
