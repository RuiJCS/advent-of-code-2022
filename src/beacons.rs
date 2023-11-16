/*


heres's the problem:
--- Day 15: Beacon Exclusion Zone ---

You feel the ground rumble again as the distress signal leads you to a large network of subterranean tunnels. You don't have time to search them all, but you don't need to: your pack contains a set of deployable sensors that you imagine were originally built to locate lost Elves.

The sensors aren't very powerful, but that's okay; your handheld device indicates that you're close enough to the source of the distress signal to use them. You pull the emergency sensor system out of your pack, hit the big button on top, and the sensors zoom off down the tunnels.

Once a sensor finds a spot it thinks will give it a good reading, it attaches itself to a hard surface and begins monitoring for the nearest signal source beacon. Sensors and beacons always exist at integer coordinates. Each sensor knows its own position and can determine the position of a beacon precisely; however, sensors can only lock on to the one beacon closest to the sensor as measured by the Manhattan distance. (There is never a tie where two beacons are the same distance to a sensor.)

It doesn't take long for the sensors to report back their positions and closest beacons (your puzzle input). For example:

Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3

So, consider the sensor at 2,18; the closest beacon to it is at -2,15. For the sensor at 9,16, the closest beacon to it is at 10,16.

Drawing sensors as S and beacons as B, the above arrangement of sensors and beacons looks like this:

               1    1    2    2
     0    5    0    5    0    5
 0 ....S.......................
 1 ......................S.....
 2 ...............S............
 3 ................SB..........
 4 ............................
 5 ............................
 6 ............................
 7 ..........S.......S.........
 8 ............................
 9 ............................
10 ....B.......................
11 ..S.........................
12 ............................
13 ............................
14 ..............S.......S.....
15 B...........................
16 ...........SB...............
17 ................S..........B
18 ....S.......................
19 ............................
20 ............S......S........
21 ............................
22 .......................B....

This isn't necessarily a comprehensive map of all beacons in the area, though. Because each sensor only identifies its closest beacon, if a sensor detects a beacon, you know there are no other beacons that close or closer to that sensor. There could still be beacons that just happen to not be the closest beacon to any sensor. Consider the sensor at 8,7:

               1    1    2    2
     0    5    0    5    0    5
-2 ..........#.................
-1 .........###................
 0 ....S...#####...............
 1 .......#######........S.....
 2 ......#########S............
 3 .....###########SB..........
 4 ....#############...........
 5 ...###############..........
 6 ..#################.........
 7 .#########S#######S#........
 8 ..#################.........
 9 ...###############..........
10 ....B############...........
11 ..S..###########............
12 ......#########.............
13 .......#######..............
14 ........#####.S.......S.....
15 B........###................
16 ..........#SB...............
17 ................S..........B
18 ....S.......................
19 ............................
20 ............S......S........
21 ............................
22 .......................B....

This sensor's closest beacon is at 2,10, and so you know there are no beacons that close or closer (in any positions marked #).

None of the detected beacons seem to be producing the distress signal, so you'll need to work out where the distress beacon is by working out where it isn't. For now, keep things simple by counting the positions where a beacon cannot possibly be along just a single row.

So, suppose you have an arrangement of beacons and sensors like in the example above and, just in the row where y=10, you'd like to count the number of positions a beacon cannot possibly exist. The coverage from all sensors near that row looks like this:

                 1    1    2    2
       0    5    0    5    0    5
 9 ...#########################...
10 ..####B######################..
11 .###S#############.###########.

In this example, in the row where y=10, there are 26 positions where a beacon cannot be present.

Consult the report from the sensors you just deployed. In the row where y=2000000, how many positions cannot contain a beacon?


*/

mod utils;

use std::cmp::max;
use std::cmp::min;

use regex::Regex;

use utils::utils::read_file;

const BEACONS_FILE_NAME: &str = "inputs/beacons.txt";
const ERROR_MESSAGE: &str = "Error reading input";

#[derive(Debug, Clone, PartialEq, Eq)]
struct SensorBeacon {
    pub sensor: (isize, isize),
    pub beacon: (isize, isize),
    pub distance: usize,
}

fn big_apple_distance(sensor_x: isize, sensor_y: isize, beacon_x: isize, beacon_y: isize) -> usize {
    sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y)
}

fn create_sensor_beacon(
    sensor_x: isize,
    sensor_y: isize,
    beacon_x: isize,
    beacon_y: isize,
) -> SensorBeacon {
    SensorBeacon {
        sensor: (sensor_x, sensor_y),
        beacon: (beacon_x, beacon_y),
        distance: big_apple_distance(sensor_x, sensor_y, beacon_x, beacon_y),
    }
}

fn parse_input(file: &String) -> (Vec<SensorBeacon>, isize, isize, isize, isize) {
    let mut sensors_beacons: Vec<SensorBeacon> = Vec::new();
    let re = Regex::new(
        r"Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)\n",
    )
    .unwrap();
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;
    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;
    for (_captured_string, [x1, y1, x2, y2]) in re.captures_iter(file.as_str()).map(|c| c.extract())
    {
        let sensor_x = x1.parse().unwrap();
        let sensor_y = y1.parse().unwrap();
        let beacon_x = x2.parse().unwrap();
        let beacon_y = y2.parse().unwrap();
        let sb = create_sensor_beacon(sensor_x, sensor_y, beacon_x, beacon_y);
        sensors_beacons.push(sb.to_owned());
        max_x = max(
            max_x,
            max(
                sensor_x + sb.distance as isize,
                beacon_x + sb.distance as isize,
            ),
        );
        min_x = min(
            min_x,
            min(
                sensor_x - sb.distance as isize,
                beacon_x - sb.distance as isize,
            ),
        );
        max_y = max(max_y, max(sensor_y, beacon_y));
        min_y = min(min_y, min(sensor_y, beacon_y));

        //calcular x possível mais á esquerda e á direita
    }
    (sensors_beacons, max_x, max_y, min_x, min_y)
}

fn main() {
    let file = read_file(BEACONS_FILE_NAME, ERROR_MESSAGE);
    let (sensors_beacons, max_x, _max_y, min_x, _min_y) = parse_input(&file);

    println!(
        "{}",
        calculate_impossible_beacons_in_row(2000000, &sensors_beacons, max_x, min_x)
    );
}

fn calculate_impossible_beacons_in_row(
    row_index: isize,
    map: &Vec<SensorBeacon>,
    max_x: isize,
    min_x: isize,
) -> usize {
    (min_x..=max_x)
        .filter(|x| {
            map.iter().any(|sb| {
                let distance: usize = big_apple_distance(sb.sensor.0, sb.sensor.1, *x, row_index);
                distance <= sb.distance && !(*x == sb.beacon.0 && row_index == sb.beacon.1)
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const BEACONS_TEST_FILE_NAME: &str = "inputs/beacons_test.txt";

    #[test]
    fn test_create_sensor() {
        let expected_sensor = SensorBeacon {
            sensor: (2, 18),
            beacon: (-2, 15),
            distance: 7,
        };
        let sensor = create_sensor_beacon(2, 18, -2, 15);
        assert_eq!(expected_sensor, sensor);
    }

    #[test]
    fn test_distance() {
        assert_eq!(4, big_apple_distance(10, 20, 10, 16));
        assert_eq!(
            1_912_059,
            big_apple_distance(167_388, 3_570_975, -1_018_858, 4_296_788)
        );
        assert_eq!(
            1_975_212,
            big_apple_distance(3_928_889, 1_064_434, 3_340_990, 2_451_747)
        );
    }

    #[test]
    fn test_parse() {
        let file = read_file(BEACONS_TEST_FILE_NAME, ERROR_MESSAGE);
        let (res, max_x, max_y, min_x, min_y) = parse_input(&file);
        let expected_vec = vec![
            create_sensor_beacon(2, 18, -2, 15),
            create_sensor_beacon(9, 16, 10, 16),
            create_sensor_beacon(13, 2, 15, 3),
            create_sensor_beacon(12, 14, 10, 16),
            create_sensor_beacon(10, 20, 10, 16),
            create_sensor_beacon(14, 17, 10, 16),
            create_sensor_beacon(8, 7, 2, 10),
            create_sensor_beacon(2, 0, 2, 10),
            create_sensor_beacon(0, 11, 2, 10),
            create_sensor_beacon(20, 14, 25, 17),
            create_sensor_beacon(17, 20, 21, 22),
            create_sensor_beacon(16, 7, 15, 3),
            create_sensor_beacon(14, 3, 15, 3),
            create_sensor_beacon(20, 1, 15, 3),
        ];
        assert_eq!(max_x, 25);
        assert_eq!(max_y, 22);
        assert_eq!(min_x, -2);
        assert_eq!(min_y, 0);
        assert_eq!(res, expected_vec);

        assert_eq!(
            26,
            calculate_impossible_beacons_in_row(10, &res, max_x, min_x)
        );
    }

    #[test]
    fn test_part1() {
        let file = read_file(BEACONS_FILE_NAME, ERROR_MESSAGE);
        let (res, max_x, _max_y, min_x, min_y) = parse_input(&file);
        println!("max_x = {}", max_x);
        println!("min_x = {}", min_x);

        assert_eq!(
            5142231,
            calculate_impossible_beacons_in_row(2_000_000, &res, max_x, min_x)
        );
    }
}
