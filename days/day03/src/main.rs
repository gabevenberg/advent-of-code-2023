mod part1;
mod part2;
mod parse;

fn main() {
    let input = include_str!("./input.txt");
    let structured_input = parse::parse(input);

    println!("Part One");
    println!("Result: {}", part1::part1(&structured_input));

    println!("Part Two");
    println!("Result: {}", part2::part2(&structured_input));
}
