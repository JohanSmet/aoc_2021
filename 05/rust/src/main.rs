struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

impl Line {
    fn is_axis_aligned(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn delta(&self) -> [i32; 2] {
        #[allow(clippy::comparison_chain)]
        [   if self.x1 < self.x2 {1} else if self.x1 > self.x2 {-1} else {0},
            if self.y1 < self.y2 {1} else if self.y1 > self.y2 {-1} else {0} ]
    }
}

fn solve(values: &[Line], include_diagonal : bool) -> usize {
    let size_x = values.iter().map(|l| l.x1.max(l.x2)).max().unwrap() + 1;
    let size_y = values.iter().map(|l| l.y1.max(l.y2)).max().unwrap() + 1;

    let mut playfield = vec![0; (size_x * size_y) as usize];

    let mut increment_playfield = |x: i32, y: i32| {
        playfield[((size_x * y) + x) as usize] += 1;
    };

    for line in values {
        if !include_diagonal && !line.is_axis_aligned() {
            continue;
        }

        let [delta_x, delta_y] = line.delta();
        let mut x = line.x1;
        let mut y = line.y1;

        while x != line.x2 || y != line.y2 {
            increment_playfield(x, y);
            x += delta_x;
            y += delta_y;
        }
        increment_playfield(x, y);
    }

    return playfield.iter().filter(|&x| *x >= 2).count();
}

fn main() {
    let input_data = include_str!("../../input.txt");

    // parse input data
    let values : Vec<Line> = input_data
                                .lines()
                                .map(|s| {
                                        let parts : Vec<&str> = s.split(&[' ', ','][..]).collect();
                                        Line {
                                            x1: parts[0].parse::<i32>().unwrap(),
                                            y1: parts[1].parse::<i32>().unwrap(),
                                            x2: parts[3].parse::<i32>().unwrap(),
                                            y2: parts[4].parse::<i32>().unwrap(),
                                        }
                                    })
                                .collect();

    // solve problems
    println!("Part 1: {}", solve(&values, false));
    println!("Part 2: {}", solve(&values, true));
}
