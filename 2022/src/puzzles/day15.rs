use std::collections::HashSet;

#[allow(dead_code)]
const EXAMPLE: &str = include_str!("../input/15_example");
const INPUT: &str = include_str!("../input/15");

fn solve_a_for(input: &str, target_y: i32) -> usize {
    let mut positions_without_beacon = HashSet::new();

    for line in input.lines() {
        let (_, line) = line.split_once("Sensor at x=").unwrap();
        let (sensor_x, line) = line.split_once(", y=").unwrap();
        let (sensor_y, line) = line.split_once(": closest beacon is at x=").unwrap();
        let (beacon_x, beacon_y) = line.split_once(", y=").unwrap();

        let sensor_x: i32 = sensor_x.parse().unwrap();
        let sensor_y: i32 = sensor_y.parse().unwrap();
        let beacon_x: i32 = beacon_x.parse().unwrap();
        let beacon_y: i32 = beacon_y.parse().unwrap();

        let distance = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();

        let no_beacon_radius = distance - (sensor_y - target_y).abs();

        if no_beacon_radius >= 0 {
            for x in sensor_x - no_beacon_radius..sensor_x + no_beacon_radius + 1 {
                if beacon_x == x && beacon_y == target_y {
                    continue;
                }
                positions_without_beacon.insert(x);
            }
        }
    }

    positions_without_beacon.len()
}

struct Sensor {
    sensor_x: i64,
    sensor_y: i64,
    beacon_x: i64,
    beacon_y: i64,
    possible_distance: i64,
}

fn solve_b_for(input: &str, range: i64) -> i64 {
    let mut sensors = vec![];

    for line in input.lines() {
        let (_, line) = line.split_once("Sensor at x=").unwrap();
        let (sensor_x, line) = line.split_once(", y=").unwrap();
        let (sensor_y, line) = line.split_once(": closest beacon is at x=").unwrap();
        let (beacon_x, beacon_y) = line.split_once(", y=").unwrap();

        let sensor_x: i64 = sensor_x.parse().unwrap();
        let sensor_y: i64 = sensor_y.parse().unwrap();
        let beacon_x: i64 = beacon_x.parse().unwrap();
        let beacon_y: i64 = beacon_y.parse().unwrap();

        let distance = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
        let possible_distance = distance + 1;

        sensors.push(Sensor {
            sensor_x,
            sensor_y,
            beacon_x,
            beacon_y,
            possible_distance,
        });
    }

    for sensor in &sensors {
        for i in 0..sensor.possible_distance {
            let possible_locations = [
                (
                    sensor.sensor_x + sensor.possible_distance - i,
                    sensor.sensor_y + i,
                ),
                (
                    sensor.sensor_x - i,
                    sensor.sensor_y + sensor.possible_distance - i,
                ),
                (
                    sensor.sensor_x - sensor.possible_distance + i,
                    sensor.sensor_y - i,
                ),
                (
                    sensor.sensor_x + i,
                    sensor.sensor_y - sensor.possible_distance + i,
                ),
            ];

            for (x, y) in possible_locations {
                if x < 0 || y < 0 || x > range || y > range {
                    continue;
                }

                let mut seen = false;

                for other_sensor in &sensors {
                    if sensor.sensor_x == other_sensor.sensor_x
                        && sensor.sensor_y == other_sensor.sensor_y
                    {
                        continue;
                    }

                    let other_sensor_beacon_range = (other_sensor.sensor_x - other_sensor.beacon_x)
                        .abs()
                        + (other_sensor.sensor_y - other_sensor.beacon_y).abs();
                    let distance_to_other_sensor =
                        (other_sensor.sensor_x - x).abs() + (other_sensor.sensor_y - y).abs();

                    if distance_to_other_sensor <= other_sensor_beacon_range {
                        seen = true;
                        break;
                    }
                }

                if !seen {
                    return x * 4000000 + y;
                }
            }
        }
    }

    panic!("no solution found");
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE, 10), 26);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT, 2000000), 5367037);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE, 20), 56000011);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT, 4000000), 11914583249288);
}

pub fn solve_a() {
    println!("{}", solve_a_for(INPUT, 2000000));
}

pub fn solve_b() {
    println!("{}", solve_b_for(INPUT, 4000000));
}
