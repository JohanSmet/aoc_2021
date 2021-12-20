type Point = [i32;3];

fn dot_product(a: &Point, b: &Point) -> i32 {
    ((b[0] - a[0]) * (b[0] - a[0])) +
    ((b[1] - a[1]) * (b[1] - a[1])) +
    ((b[2] - a[2]) * (b[2] - a[2]))
}

fn subtract(a: &Point, b: &Point) -> Point {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    (a[0] - b[0]).abs() +
    (a[1] - b[1]).abs() +
    (a[2] - b[2]).abs()
}

fn points_rotate_x(points: &mut Vec<Point>) {
    for p in points {
        *p = [p[0], -p[2], p[1]];
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    origin: Point,
    beacons: Vec<Point>,
    dists: Vec<i32>
}

impl Scanner {

    pub fn rebuild_distance_list(&mut self) {
        self.dists.clear();
        for i in 0..self.beacons.len() {
            for j in i+1..self.beacons.len() {
                self.dists.push(dot_product(&self.beacons[i], &self.beacons[j]));
            }
        }
        self.dists.sort_unstable();
    }

    pub fn count_matching_distances(&self, other: &Scanner) -> usize {
        let mut count = 0;
        let mut o_iter = other.dists.iter();

        if let Some(mut current_o) = o_iter.next() {
            for current_s in &self.dists {
                while current_o < current_s {
                    current_o = match o_iter.next() {
                        Some(current_o) => current_o,
                        None => return count,
                    };
                }
                if current_s == current_o {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn try_merge(&mut self, rotated: &[Point], other: &mut Scanner) -> bool {

        for i in 0..self.beacons.len() {
            for j in 0..rotated.len() {
                let mut count = 0;
                let delta = subtract(&rotated[j], &self.beacons[i]);

                for p in rotated.iter().map(|p| subtract(p, &delta)) {
                    if self.beacons.contains(&p) {
                        count += 1;
                    }
                }

                if count >= 12 {
                    other.origin = subtract(&[0; 3], &delta);
                    self.beacons.extend(rotated.iter().map(|p| subtract(p, &delta)));
                    self.beacons.sort_unstable();
                    self.beacons.dedup();
                    self.rebuild_distance_list();
                    return true;
                }
            }
        }

        false
    }

    pub fn merge(&mut self, other: &mut Scanner) -> bool {

        // don't try if not enough distances between beacons match (at least twelve)
        let count = self.count_matching_distances(other);
        if count < 66 {
            return false;
        }

        let mut check_x_rotations = |beacons: &mut Vec<Point>, other: &mut Scanner| -> bool {
            for _ in 0..4 {
                if self.try_merge(beacons, other) {
                    return true;
                }
                points_rotate_x(beacons);
            }
            false
        };

        // try to merge in all orientations
        let mut beacons = other.beacons.clone();
        if check_x_rotations(&mut beacons, other) {
            return true;
        }

        let mut beacons : Vec<Point> = other.beacons.iter().map(|p| [-p[0], p[2], p[1]]).collect();
        if check_x_rotations(&mut beacons, other) {
            return true;
        }

        let mut beacons : Vec<Point> = other.beacons.iter().map(|p| [-p[1], p[0], p[2]]).collect();
        if check_x_rotations(&mut beacons, other) {
            return true;
        }

        let mut beacons : Vec<Point> = other.beacons.iter().map(|p| [p[1], p[2], p[0]]).collect();
        if check_x_rotations(&mut beacons, other) {
            return true;
        }

        let mut beacons : Vec<Point> = other.beacons.iter().map(|p| [p[2], p[0], p[1]]).collect();
        if check_x_rotations(&mut beacons, other) {
            return true;
        }

        let mut beacons : Vec<Point> = other.beacons.iter().map(|p| [-p[2], p[1], p[0]]).collect();
        if check_x_rotations(&mut beacons, other) {
            return true;
        }

        false
    }
}

fn main() {
    let input_data = include_str!("../../input.txt").trim_end();

    // parse input data
    let mut data : Vec<Scanner> = input_data.split("\n\n")
         .map(|s| {
            Scanner {
                origin: [0; 3],
                beacons: s.lines()
                    .skip(1)            // skip scanner id
                    .map(|l| {
                        let mut parts = l.split(',').map(|x| x.parse::<i32>().unwrap());
                        [parts.next().unwrap(), parts.next().unwrap(), parts.next().unwrap()]
                    })
                    .collect(),
                dists: vec![]

            }
        }).collect();

    for d in &mut data {
        d.rebuild_distance_list();
    }

    // merge scanner data into one big map
    let mut complete_map = data[0].clone();
    let mut count_merged = 1;

    while count_merged < data.len() {
        for scanner in data.iter_mut().skip(1) {
            if scanner.origin == [0; 3] && complete_map.merge(scanner) {
                count_merged += 1;
            }
        }
    }

    println!("Part 1: there are {} beacons", complete_map.beacons.len());

    let mut max_manhattan = 0;

    for i in 0..data.len() {
        for j in i+1..data.len() {
            max_manhattan = std::cmp::max(max_manhattan, manhattan_distance(&data[i].origin, &data[j].origin));
        }
    }

    println!("Part 2: maximum distance between scanners = {}", max_manhattan);
}
