use std::{cmp, vec};

type Cuboid = [[i32; 2]; 3];

#[derive(Debug)]
struct Step {
    state: bool,
    cuboid: Cuboid,
}

fn part1(boot_steps: &[Step]) {

    let mut reactor = [[[false; 101]; 101]; 101];

    for step in boot_steps {

        for x in cmp::max(-50, step.cuboid[0][0])..cmp::min(50, step.cuboid[0][1])+1 {
            for y in cmp::max(-50, step.cuboid[1][0])..cmp::min(50, step.cuboid[1][1])+1 {
                for z in cmp::max(-50, step.cuboid[2][0])..cmp::min(50, step.cuboid[2][1])+1 {
                    reactor[(x+50) as usize][(y+50) as usize][(z+50) as usize] = step.state;
                }
            }
        }
    }

    println!("Part 1: {}", reactor.iter().flatten().flatten().filter(|c| **c).count());
}

fn range_contains(range: &[i32;2], value: i32) -> bool {
    value >= range[0] && value <= range[1]
}

fn cuboid_contains_other(cuboid: &Cuboid, other: &Cuboid) -> bool {
    range_contains(&cuboid[0], other[0][0]) && range_contains(&cuboid[0], other[0][1]) &&
    range_contains(&cuboid[1], other[1][0]) && range_contains(&cuboid[1], other[1][1]) &&
    range_contains(&cuboid[2], other[2][0]) && range_contains(&cuboid[2], other[2][1])
}

fn cuboids_subtract(c1: &Cuboid, c2: &Cuboid) -> Vec<Cuboid> {
// Ok, so this isn't the most efficient way of implementing this, but it got me the gold star in a reasonable time
// and I just want to finish this thing.

    let mut to_process = vec![*c1];

    // split
    for axis in 0..3 {
        let mut new_cuboids = vec![];

        for subject in &to_process {

            let p0_splits = range_contains(&subject[axis], c2[axis][0]);
            let p1_splits = range_contains(&subject[axis], c2[axis][1]);

            if p0_splits && p1_splits {
                // total overlap -> split into three parts
                let mut part1 : Cuboid = *subject;
                part1[axis][1] = c2[axis][0] - 1;
                if part1[axis][0] <= part1[axis][1] {
                    new_cuboids.push(part1);
                }

                let mut part2 : Cuboid = *subject;
                part2[axis] = c2[axis];
                if part2[axis][0] <= part2[axis][1] {
                    new_cuboids.push(part2);
                }

                let mut part3 = *subject;
                part3[axis][0] = c2[axis][1] + 1;
                if part3[axis][0] <= part3[axis][1] {
                    new_cuboids.push(part3);
                }
            } else if !p0_splits && !p1_splits {
                // no overlap
                new_cuboids.push(*subject);
            } else {
                let p = if p0_splits {0} else {1};

                let mut part1 : Cuboid = *subject;
                part1[axis][1] = c2[axis][p] - if p == 0 {1} else {0};
                if part1[axis][0] <= part1[axis][1] {
                    new_cuboids.push(part1);
                }

                let mut part2 = *subject;
                part2[axis][0] = c2[axis][p] + if p == 1 {1} else {0};
                if part2[axis][0] <= part2[axis][1] {
                    new_cuboids.push(part2);
                }
            }
        }

        to_process = new_cuboids;
    }

    // remove cuboids that are entirely inside c2
    to_process.retain(|c| !cuboid_contains_other(c2, c));
    to_process
}

fn part2(boot_steps: &[Step]) {

    assert!(boot_steps[0].state);
    let mut reactor = vec![boot_steps[0].cuboid];
    let mut step_count = 0;

    for step in boot_steps.iter().skip(1) {

        let mut new_reactor = vec![];

        for c in &reactor {
            new_reactor.extend_from_slice(&cuboids_subtract(c, &step.cuboid));
        }

        if step.state {
            new_reactor.push(step.cuboid);
        }

        reactor = new_reactor;
        step_count += 1;
        println!("Step {} / {} done", step_count, boot_steps.len());
    }

    let count = reactor.iter().map(|c| (c[0][1] - c[0][0] + 1) as i64 *
                                       (c[1][1] - c[1][0] + 1) as i64 *
                                       (c[2][1] - c[2][0] + 1) as i64).sum::<i64>();
    println!("Part 2: {}", count);
}


fn main() {
    let input_data = include_str!("../../input.txt").trim_end();

    fn parse_range(range: &str) -> [i32; 2] {
        let mut ns = range.split("..").map(|x| x.parse::<i32>().unwrap());
        [ns.next().unwrap(), ns.next().unwrap()]
    }

    let steps : Vec<Step> = input_data
                                .lines()
                                .map(|l| {
                                    let mut parts_iter = l.split(&[' ', ','][..]);
                                    let str_state = parts_iter.next().unwrap();

                                    Step {
                                        state: str_state == "on",
                                        cuboid: [
                                            parse_range(&parts_iter.next().unwrap()[2..]),
                                            parse_range(&parts_iter.next().unwrap()[2..]),
                                            parse_range(&parts_iter.next().unwrap()[2..]),
                                        ]
                                    }

                                    })
                                .collect();

    part1(&steps);
    part2(&steps);
}
