const TEST_RUN: bool = false;
const TARGET_X: [i32; 2] = if TEST_RUN {[20, 30]} else {[277,318]};
const TARGET_Y: [i32; 2] = if TEST_RUN {[-10, -5]} else {[-92,-53]};

fn solve(mut vel_x: i32, mut vel_y: i32) -> (bool, i32) {

    let mut x = 0;
    let mut y = 0;
    let mut max_y = 0;

    let accel_x = if vel_x > 0 {-1} else {1};

    loop {
        x += vel_x;
        y += vel_y;

        if vel_x != 0 {
            vel_x += accel_x;
        }

        vel_y -= 1;

        if y > max_y {
            max_y = y;
        }

        // target area hit?
        if x >= TARGET_X[0] && x <= TARGET_X[1] && y >= TARGET_Y[0] && y <= TARGET_Y[1] {
            return (true, max_y);
        }

        // target unreachable ?
        if (vel_x > 0 && x > TARGET_X[1]) || (vel_x < 0 && x < TARGET_X[0]) || y < TARGET_Y[0] {
            return (false, max_y);
        }
    }
}


fn main() {

    let mut max_y = 0;
    let mut hit_count = 0;

    for vel_y in -500..500 {
        for vel_x in 0..500 {
            let (hit, top) = solve(vel_x, vel_y);
            if hit {
                hit_count += 1;
                if top > max_y {
                    max_y = top;
                }
            }
        }
    }

    println!("Part 1: Highest y = {}", max_y);
    println!("Part 2: Hit count = {}", hit_count);
}
