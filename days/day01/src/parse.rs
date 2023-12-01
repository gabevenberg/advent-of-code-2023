static REPLACEMENT_STRINGS: [(&str, &str); 10] = [
    ("zero", "zero0zero"),
    ("one", "one1one"),
    ("two", "two2two"),
    ("three", "three3three"),
    ("four", "four4four"),
    ("five", "five5five"),
    ("six", "six6six"),
    ("seven", "seven7seven"),
    ("eight", "eight8eight"),
    ("nine", "nine9nine"),
];

pub fn parse_english(input: &str) -> Vec<Vec<char>> {
    input
        .to_string()
        .lines()
        .map(|l| {
            let mut ret = l.to_string();
            for (string, replacement) in REPLACEMENT_STRINGS.iter() {
                ret = ret.replace(string, replacement)
            }
            ret
        })
        .map(|l| l.chars().filter(|c| c.is_ascii_digit()).collect())
        .collect()
}

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().filter(|c| c.is_ascii_digit()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = concat!("1abc2\n", "pqr3stu8vwx\n", "a1b2c3d4e5f\n", "treb7uchet\n",);
        assert_eq!(
            parse(input),
            vec![
                vec!['1', '2',],
                vec!['3', '8',],
                vec!['1', '2', '3', '4', '5'],
                vec!['7']
            ]
        );
    }
    #[test]
    fn test_english() {
        let input = concat!(
            "two1nine\n",
            "eightwothree\n",
            "abcone2threexyz\n",
            "xtwone3four\n",
            "4nineeightseven2\n",
            "zoneight234\n",
            "7pqrstsixteen\n",
        );
        assert_eq!(
            parse_english(input),
            vec![
                vec!['2', '1', '9',],
                vec!['8', '2', '3',],
                vec!['1', '2', '3',],
                vec!['2', '1', '3', '4'],
                vec!['4', '9', '8', '7', '2'],
                vec!['1', '8', '2', '3', '4'],
                vec!['7', '6'],
            ]
        );
    }
}
