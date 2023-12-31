use crate::parse::*;

pub fn part2(input: &(Vec<u64>, Vec<Map>)) -> usize {
    let mut seeds = convert_seeds_to_ranges(&input.0);
    for map in &input.1{
        seeds = seeds.into_iter().flat_map(|s|{
            map.map_ranges(s)
        }).collect();
    };
    seeds.iter().map(|s| s.start).min().unwrap() as usize
}

fn convert_seeds_to_ranges(seeds: &[u64]) -> Vec<SeedRange> {
    seeds.chunks(2).map(|c| c[0]..(c[0] + c[1])).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_seeds_to_ranges() {
        let input = [79, 14, 55, 13];
        assert_eq!(convert_seeds_to_ranges(&input), vec![79..93, 55..68])
    }

    #[test]
    fn test_part2() {
        let input = (
            vec![79, 14, 55, 13],
            vec![
                Map {
                    from: "seed".to_string(),
                    to: "soil".to_string(),
                    ranges: vec![
                        MapRange {
                            dest_start: 50,
                            src_start: 98,
                            len: 2,
                        },
                        MapRange {
                            dest_start: 52,
                            src_start: 50,
                            len: 48,
                        },
                    ],
                },
                Map {
                    from: "soil".to_string(),
                    to: "fertilizer".to_string(),
                    ranges: vec![
                        MapRange {
                            dest_start: 0,
                            src_start: 15,
                            len: 37,
                        },
                        MapRange {
                            dest_start: 37,
                            src_start: 52,
                            len: 2,
                        },
                        MapRange {
                            dest_start: 39,
                            src_start: 0,
                            len: 15,
                        },
                    ],
                },
                Map {
                    from: "fertilizer".to_string(),
                    to: "water".to_string(),
                    ranges: vec![
                        MapRange {
                            dest_start: 49,
                            src_start: 53,
                            len: 8,
                        },
                        MapRange {
                            dest_start: 0,
                            src_start: 11,
                            len: 42,
                        },
                        MapRange {
                            dest_start: 42,
                            src_start: 0,
                            len: 7,
                        },
                        MapRange {
                            dest_start: 57,
                            src_start: 7,
                            len: 4,
                        },
                    ],
                },
                Map {
                    from: "water".to_string(),
                    to: "light".to_string(),
                    ranges: vec![
                        MapRange {
                            dest_start: 88,
                            src_start: 18,
                            len: 7,
                        },
                        MapRange {
                            dest_start: 18,
                            src_start: 25,
                            len: 70,
                        },
                    ],
                },
                Map {
                    from: "light".to_string(),
                    to: "temperature".to_string(),
                    ranges: vec![
                        MapRange {
                            dest_start: 45,
                            src_start: 77,
                            len: 23,
                        },
                        MapRange {
                            dest_start: 81,
                            src_start: 45,
                            len: 19,
                        },
                        MapRange {
                            dest_start: 68,
                            src_start: 64,
                            len: 13,
                        },
                    ],
                },
                Map {
                    from: "temperature".to_string(),
                    to: "humidity".to_string(),
                    ranges: vec![
                        MapRange {
                            dest_start: 0,
                            src_start: 69,
                            len: 1,
                        },
                        MapRange {
                            dest_start: 1,
                            src_start: 0,
                            len: 69,
                        },
                    ],
                },
                Map {
                    from: "humidity".to_string(),
                    to: "location".to_string(),
                    ranges: vec![
                        MapRange {
                            dest_start: 60,
                            src_start: 56,
                            len: 37,
                        },
                        MapRange {
                            dest_start: 56,
                            src_start: 93,
                            len: 4,
                        },
                    ],
                },
            ],
        );
        assert_eq!(part2(&input), 46);
    }
}
