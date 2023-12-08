use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Node {
    pub left: String,
    pub right: String,
}

static NODE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([[:alpha:]]{3}) = \(([[:alpha:]]{3}), ([[:alpha:]]{3})\)$").unwrap()
});

pub fn parse(input: &str) -> (Vec<Direction>, HashMap<String, Node>) {
    let mut lines = input.lines();
    let mut directions = Vec::new();
    //parse the directions
    let dirline = lines.next().unwrap();
    for char in dirline.chars() {
        directions.push(match char {
            'L' => Direction::Left,
            'R' => Direction::Right,
            e => panic!("unexpected direction {}", e),
        })
    }
    //skip a blank line
    lines.next();

    // process the rest of the lines
    let mut graph = HashMap::new();
    for line in lines {
        let captures = NODE_REGEX.captures(line).unwrap();
        graph.insert(
            captures[1].to_string(),
            Node {
                left: captures[2].to_string(),
                right: captures[3].to_string(),
            },
        );
    }
    (directions, graph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use collection_literals::collection;

    #[test]
    fn test_parse() {
        let input = concat!(
            "LLR\n",
            "\n",
            "AAA = (BBB, BBB)\n",
            "BBB = (AAA, ZZZ)\n",
            "ZZZ = (ZZZ, ZZZ)\n",
        );
        println!("{:#?}", parse(input));
        assert_eq!(
            parse(input),
            (
                vec![Direction::Left, Direction::Left, Direction::Right],
                collection! {
                "AAA".to_string() => Node{ left: "BBB".to_string(), right: "BBB".to_string() },
                "BBB".to_string() => Node{ left: "AAA".to_string(), right: "ZZZ".to_string() },
                "ZZZ".to_string() => Node{ left: "ZZZ".to_string(), right: "ZZZ".to_string() },
                }
            )
        );
    }
}
