#[derive(Debug)]
struct Fold {
    axis: usize,
    at: i32
}

fn fold_coords(coords: &mut Vec<[i32;2]>, fold: &Fold) {
    for coord in &mut *coords {
        if coord[fold.axis] > fold.at {
            coord[fold.axis] = fold.at - (coord[fold.axis] - fold.at);
        }
    }

    coords.sort_unstable();
    coords.dedup();
}

fn display(coords: &[[i32; 2]]) {

    let mut buffer: [[char; 40]; 6] = [[' '; 40]; 6];

    for coord in coords {
        buffer[coord[1] as usize][coord[0] as usize] = '*';
    }

    for line in buffer {
        println!("{}", line.iter().collect::<String>());
    }
}

fn main() {
    let mut input_iter = include_str!("../../input.txt").trim_end().lines();

    let mut coords : Vec<[i32; 2]> = vec![];
    let mut folds: Vec<Fold> = vec![];

    // read input data - coordinates
    let mut line = input_iter.next().expect("Invalid input");
    while !line.is_empty() {
        let mut input_coords = line.split(',').map(|l| l.parse::<i32>().unwrap_or(0));
        coords.push([
            input_coords.next().unwrap(),
            input_coords.next().unwrap()
        ]);

        line = input_iter.next().expect("Invalid input");
    }

    // read input data - folds
    let mut line = input_iter.next();
    while line.is_some() {
        let mut parts = line.unwrap().split('=');
        folds.push(Fold {
            axis: if parts.next() == Some("fold along x") {0} else {1},
            at: parts.next().unwrap().parse::<i32>().unwrap_or(0),
        });

        line = input_iter.next();
    }

    // part 1: fold once
    fold_coords(&mut coords, &folds[0]);
    println!("Part 1: {} dots", coords.len());

    // part 2: keep folding
    for fold in &folds[1..] {
        fold_coords(&mut coords, fold);
    }
    display(&coords);

}
