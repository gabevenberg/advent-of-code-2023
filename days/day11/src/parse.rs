use std::collections::HashSet;

use aoc_libs::distances::Distances;
use aoc_libs::points::UPoint;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Space {
    Galaxy,
    Space,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SparseSpace {
    galaxies: HashSet<UPoint>,
    empty_rows: HashSet<usize>,
    empty_columns: HashSet<usize>,
}

impl SparseSpace {
    pub fn get_distance_between_galaxies(
        &self,
        galaxy1: &UPoint,
        galaxy2: &UPoint,
        expantion: &usize,
    ) -> usize {
        // println!("distance between {:?} and {:?}", galaxy1, galaxy2);

        //get unexpanded distance.
        let mut distance = galaxy1.taxicab_distance(galaxy2);
        // println!("unexpanded distance is {}", distance);

        //expand rows
        for y in galaxy1.y.min(galaxy2.y)+1..galaxy1.y.max(galaxy2.y) {
            if self.empty_rows.contains(&y) {
                // println!("expanding row {}", y);
                distance += expantion
            }
        }

        //expand columns.
        for x in galaxy1.x.min(galaxy2.x)+1..galaxy1.x.max(galaxy2.x) {
            if self.empty_columns.contains(&x) {
                // println!("expanding col {}", x);
                distance += expantion
            }
        }
        distance
    }

    pub fn get_sum_of_distances(&self, expantion: usize) -> usize {
        self.galaxies
            .iter()
            .combinations(2)
            // .inspect(|pair| println!("{:?}", pair))
            .map(|pair| self.get_distance_between_galaxies(pair[0], pair[1], &expantion))
            // .inspect(|distance| println!("{:?}", distance))
            .sum()
    }
}

impl From<Vec<Vec<Space>>> for SparseSpace {
    fn from(value: Vec<Vec<Space>>) -> Self {
        let mut galaxies: HashSet<UPoint> = HashSet::new();
        let mut empty_rows: HashSet<usize> = HashSet::new();
        let mut empty_columns: HashSet<usize> = HashSet::new();
        //find all galaxies and empty rows.
        for (y, row) in value.iter().enumerate() {
            let mut has_galaxy = false;
            for (x, space) in row.iter().enumerate() {
                if *space == Space::Galaxy {
                    has_galaxy = true;
                    galaxies.insert(UPoint { x, y });
                };
            }
            if !has_galaxy {
                empty_rows.insert(y);
            };
        }

        //find all empty columns
        for x in 0..value[0].len() {
            let mut has_galaxy = false;
            for y in 0..value.len() {
                if value[y][x] == Space::Galaxy {
                    has_galaxy = true;
                }
            }
            if !has_galaxy {
                empty_columns.insert(x);
            };
        }

        SparseSpace {
            galaxies,
            empty_rows,
            empty_columns,
        }
    }
}

pub fn parse(input: &str) -> SparseSpace {
    parse_to_space(input).into()
}

pub fn parse_to_space(input: &str) -> Vec<Vec<Space>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Space::Galaxy,
                    '.' => Space::Space,
                    _ => panic!("unexpected char {}", c),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use collection_literals::collection;

    use super::*;

    #[test]
    fn test_get_sum_of_distances() {
        let tested = SparseSpace {
            galaxies: collection! {
                UPoint { x: 0, y: 2 },
                UPoint { x: 0, y: 9 },
                UPoint { x: 1, y: 5 },
                UPoint { x: 3, y: 0 },
                UPoint { x: 4, y: 9 },
                UPoint { x: 6, y: 4 },
                UPoint { x: 7, y: 1 },
                UPoint { x: 7, y: 8 },
                UPoint { x: 9, y: 6 }
            },
            empty_rows: collection! {3, 7},
            empty_columns: collection! {2, 5, 8},
        };
        assert_eq!(tested.get_sum_of_distances(1), 374)
    }

    #[test]
    fn test_get_distance_between_galaxies() {
        let tested = SparseSpace {
            galaxies: collection! {
                UPoint { x: 0, y: 2 },
                UPoint { x: 0, y: 9 },
                UPoint { x: 1, y: 5 },
                UPoint { x: 3, y: 0 },
                UPoint { x: 4, y: 9 },
                UPoint { x: 6, y: 4 },
                UPoint { x: 7, y: 1 },
                UPoint { x: 7, y: 8 },
                UPoint { x: 9, y: 6 }
            },
            empty_rows: collection! {3, 7},
            empty_columns: collection! {2, 5, 8},
        };
        assert_eq!(
            tested.get_distance_between_galaxies(
                &UPoint { x: 4, y: 9 },
                &UPoint { x: 1, y: 5 },
                &1
            ),
            9
        );
        assert_eq!(
            tested.get_distance_between_galaxies(
                &UPoint { x: 7, y: 8 },
                &UPoint { x: 0, y: 3 },
                &1
            ),
            15
        );
        assert_eq!(
            tested.get_distance_between_galaxies(
                &UPoint { x: 0, y: 2 },
                &UPoint { x: 9, y: 6 },
                &1
            ),
            17
        );
        assert_eq!(
            tested.get_distance_between_galaxies(
                &UPoint { x: 0, y: 9 },
                &UPoint { x: 4, y: 9 },
                &1
            ),
            5
        );
        assert_eq!(
            tested.get_distance_between_galaxies(
                &UPoint { x: 3, y: 0 },
                &UPoint { x: 6, y: 4 },
                &1
            ),
            9
        );
        assert_eq!(
            tested.get_distance_between_galaxies(
                &UPoint { x: 7, y: 8 },
                &UPoint { x: 7, y: 8 },
                &1
            ),
            0
        );
    }

    #[test]
    fn test_parse() {
        let input = concat!(
            "...#......\n",
            ".......#..\n",
            "#.........\n",
            "..........\n",
            "......#...\n",
            ".#........\n",
            ".........#\n",
            "..........\n",
            ".......#..\n",
            "#...#.....\n",
        );
        assert_eq!(
            parse(input),
            SparseSpace {
                galaxies: collection! {
                    UPoint { x: 0, y: 2 },
                    UPoint { x: 0, y: 9 },
                    UPoint { x: 1, y: 5 },
                    UPoint { x: 3, y: 0 },
                    UPoint { x: 4, y: 9 },
                    UPoint { x: 6, y: 4 },
                    UPoint { x: 7, y: 1 },
                    UPoint { x: 7, y: 8 },
                    UPoint { x: 9, y: 6 }
                },
                empty_rows: collection! {3, 7},
                empty_columns: collection! {2, 5, 8}
            }
        )
    }

    #[test]
    fn test_parse_to_space() {
        let input = concat!(
            "...#......\n",
            ".......#..\n",
            "#.........\n",
            "..........\n",
            "......#...\n",
            ".#........\n",
            ".........#\n",
            "..........\n",
            ".......#..\n",
            "#...#.....\n",
        );
        assert_eq!(
            parse_to_space(input),
            vec![
                vec![
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Galaxy,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space
                ],
                vec![
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Galaxy,
                    Space::Space,
                    Space::Space
                ],
                vec![
                    Space::Galaxy,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space
                ],
                vec![
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space
                ],
                vec![
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Galaxy,
                    Space::Space,
                    Space::Space,
                    Space::Space
                ],
                vec![
                    Space::Space,
                    Space::Galaxy,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space
                ],
                vec![
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Galaxy
                ],
                vec![
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space
                ],
                vec![
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Galaxy,
                    Space::Space,
                    Space::Space
                ],
                vec![
                    Space::Galaxy,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Galaxy,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space,
                    Space::Space
                ]
            ]
        );
    }
}
