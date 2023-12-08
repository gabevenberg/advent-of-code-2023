use std::collections::HashMap;

use crate::parse::*;

pub fn part1(input: &(Vec<Direction>, HashMap<String, Node>)) -> usize {
    let mut nodes_visited: usize = 0;
    let mut current_node: String = "AAA".to_string();
    while current_node != *"ZZZ" {
        let direction = input.0[nodes_visited % input.0.len()];
        match direction {
            Direction::Left => current_node = input.1.get(&current_node).unwrap().left.clone(),
            Direction::Right => current_node = input.1.get(&current_node).unwrap().right.clone(),
        };
        nodes_visited += 1;
    }
    nodes_visited
}

#[cfg(test)]
mod tests {
    use collection_literals::collection;

    use super::*;

    #[test]
    fn test_part1() {
        let input: (Vec<Direction>, HashMap<String, Node>) = (
            vec![Direction::Left, Direction::Left, Direction::Right],
            collection! {
            "AAA".to_string() => Node{ left: "BBB".to_string(), right: "BBB".to_string() },
            "BBB".to_string() => Node{ left: "AAA".to_string(), right: "ZZZ".to_string() },
            "ZZZ".to_string() => Node{ left: "ZZZ".to_string(), right: "ZZZ".to_string() },
            },
        );
        assert_eq!(part1(&input), 6);
    }
}
