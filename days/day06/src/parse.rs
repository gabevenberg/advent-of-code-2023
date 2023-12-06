use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Race {
    pub time: u64,
    pub record: u64,
}

impl Race {
    pub fn distance_given_charge_time(&self, charge_time: u64) -> u64 {
        if charge_time <= self.time {
            charge_time * (self.time - charge_time)
        } else {
            0
        }
    }

    pub fn num_ways_to_win(&self) -> u64 {
        // since distance = charge(time-charge),
        // we can rearrange into charge^2-(time)charge + distance = 0.
        // if we set distance to record+1 (the min distance needed to win), we can use the quadratic formula, where
        // a=1
        // also notice that the upper and lower bound of charge times always sums up to
        // the total time, so we can compute the lower bound from the upper bound.
        // (too lazy to prove this...)
        let b = -(self.time as f64);
        let c = (self.record + 1) as f64;
        let upper_bound = ((-b + (b.powi(2) - 4.0 * c).sqrt()) / 2.0).floor() as u64;
        let lower_bound = self.time - upper_bound;
        println!(
            "upper bound is {}, lower bound is {}",
            upper_bound, lower_bound
        );
        // off by one because if your upper and lower bounds are the same, there is 1 way to win.
        upper_bound - lower_bound + 1
    }
}

pub fn parse(input: &str) -> Vec<Race> {
    let times: IResult<&str, Vec<u64>> = terminated(
        preceded(
            preceded(tag("Time:"), multispace0),
            separated_list1(multispace0, nom::character::complete::u64),
        ),
        multispace0,
    )(input);
    let (input, times) = times.unwrap();
    let distances: IResult<&str, Vec<u64>> = terminated(
        preceded(
            preceded(tag("Distance:"), multispace0),
            separated_list1(multispace0, nom::character::complete::u64),
        ),
        multispace0,
    )(input);
    let (input, distances) = distances.unwrap();
    assert_eq!(input, "");
    times
        .into_iter()
        .zip(distances)
        .map(|r| Race {
            time: r.0,
            record: r.1,
        })
        .collect()
}

pub fn part2_parse(input: &str) -> Race {
    let mut string = input.to_string();
    string.retain(|c| c != ' ');
    let time: IResult<&str, u64> = terminated(
        preceded(tag("Time:"), nom::character::complete::u64),
        multispace0,
    )(&string);
    let (input, time) = time.unwrap();
    let distance: IResult<&str, u64> = terminated(
        preceded(tag("Distance:"), nom::character::complete::u64),
        multispace0,
    )(input);
    let (input, distance) = distance.unwrap();
    assert_eq!(input, "");
    Race {
        time,
        record: distance,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_ways_to_win() {
        let input = Race { time: 7, record: 9 };
        assert_eq!(input.num_ways_to_win(), 4);
        let input = Race {
            time: 15,
            record: 40,
        };
        assert_eq!(input.num_ways_to_win(), 8);
        let input = Race {
            time: 30,
            record: 200,
        };
        assert_eq!(input.num_ways_to_win(), 9);
        let input = Race {
            time: 71530,
            record: 940200,
        };
        assert_eq!(input.num_ways_to_win(), 71503);
    }

    #[test]
    fn test_distance_given_charge_time() {
        let input = Race { time: 7, record: 9 };
        assert_eq!(input.distance_given_charge_time(0), 0);
        assert_eq!(input.distance_given_charge_time(1), 6);
        assert_eq!(input.distance_given_charge_time(2), 10);
        assert_eq!(input.distance_given_charge_time(3), 12);
        assert_eq!(input.distance_given_charge_time(4), 12);
        assert_eq!(input.distance_given_charge_time(5), 10);
        assert_eq!(input.distance_given_charge_time(6), 6);
        assert_eq!(input.distance_given_charge_time(7), 0);
    }

    #[test]
    fn test_parse_part2() {
        let input = concat!("Time:      7  15   30\n", "Distance:  9  40  200\n",);
        assert_eq!(
            part2_parse(input),
            Race {
                time: 71530,
                record: 940200
            }
        );
    }

    #[test]
    fn test_parse() {
        let input = concat!("Time:      7  15   30\n", "Distance:  9  40  200\n",);
        assert_eq!(
            parse(input),
            vec![
                Race { time: 7, record: 9 },
                Race {
                    time: 15,
                    record: 40
                },
                Race {
                    time: 30,
                    record: 200
                },
            ]
        );
    }
}
