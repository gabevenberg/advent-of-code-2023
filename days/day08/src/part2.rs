use aoc_libs::misc::arr_lcm;
use std::collections::HashMap;

use crate::parse::*;

// note about inputs: each "ends with A" node is the starting point,
// and is not returned to part of the loop.
// eatch 'Z' node is at the 'end' of the loop,
// so the time when you first hit z is equal to your cycle time.

pub fn part2(input: &(Vec<Direction>, HashMap<String, Node>)) -> usize {
    let (directions, graph) = input;
    let starting_points = find_starting_points(graph);
    println!("{:?}", starting_points);
    let cycle_lengths: Vec<_> = starting_points
        .iter()
        .inspect(|p| print!("starting point {}: ", p))
        .map(|p| cycle_len_and_offset(p, directions, graph))
        .inspect(|c| println!("cycles: {}", c))
        .collect();
    arr_lcm(&cycle_lengths)
}

//returns the length of the loop
fn cycle_len_and_offset(
    start: &str,
    directions: &[Direction],
    graph: &HashMap<String, Node>,
) -> usize {
    let mut current_node: String = start.to_string();
    let mut dir_index: usize = 0;
    let mut cycles: usize = 0;
    while !current_node.ends_with('Z') {
        match directions[dir_index % directions.len()] {
            Direction::Left => current_node = graph.get(&current_node).unwrap().left.clone(),
            Direction::Right => current_node = graph.get(&current_node).unwrap().right.clone(),
        };
        cycles += 1;
        dir_index += 1;
        // println!("finding index: {}", current_node);
    }
    cycles
}

fn find_starting_points(input: &HashMap<String, Node>) -> Vec<String> {
    let mut ret = Vec::new();
    for point in input.keys() {
        if point.ends_with('A') {
            ret.push(point.to_string())
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_len_of_cycle() {
        let input = parse(concat!(
            "LR\n",
            "\n",
            "11A = (11B, XXX)\n",
            "11B = (XXX, 11Z)\n",
            "11Z = (11B, XXX)\n",
            "22A = (22B, XXX)\n",
            "22B = (22C, 22C)\n",
            "22C = (22Z, 22Z)\n",
            "22Z = (22B, 22B)\n",
            "XXX = (XXX, XXX)\n",
        ));
        assert_eq!(cycle_len_and_offset("11A", &input.0, &input.1), 2);
        assert_eq!(cycle_len_and_offset("22A", &input.0, &input.1), 3);
    }

    #[test]
    fn test_find_starting_points() {
        let input = parse(concat!(
            "LR\n",
            "\n",
            "11A = (11B, XXX)\n",
            "11B = (XXX, 11Z)\n",
            "11Z = (11B, XXX)\n",
            "22A = (22B, XXX)\n",
            "22B = (22C, 22C)\n",
            "22C = (22Z, 22Z)\n",
            "22Z = (22B, 22B)\n",
            "XXX = (XXX, XXX)\n",
        ));
        let starting_points = find_starting_points(&input.1);
        assert_eq!(starting_points.len(), 2);
        assert!(starting_points.contains(&"11A".to_string()));
        assert!(starting_points.contains(&"22A".to_string()));
    }

    #[test]
    fn test_part2() {
        let input = parse(concat!(
            "LR\n",
            "\n",
            "11A = (11B, XXX)\n",
            "11B = (XXX, 11Z)\n",
            "11Z = (11B, XXX)\n",
            "22A = (22B, XXX)\n",
            "22B = (22C, 22C)\n",
            "22C = (22Z, 22Z)\n",
            "22Z = (22B, 22B)\n",
            "XXX = (XXX, XXX)\n",
        ));
        assert_eq!(part2(&input), 6);
    }
}
