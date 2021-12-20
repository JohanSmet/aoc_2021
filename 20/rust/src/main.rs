fn sample_pixel(img: &[String], x: i32, y: i32, fill: usize) -> usize {
    if x < 0 || y < 0 || y >= img.len() as i32 || x >= img[y as usize].len() as i32 {
        fill
    } else if img[y as usize].chars().nth(x as usize) == Some('#') {
        1
    } else {
        0
    }
}

fn filter_index(img: &[String], x: i32, y: i32, fill: usize) -> usize {
    let mut result = sample_pixel(img, x - 1, y - 1, fill);
    result = (result << 1) + sample_pixel(img, x, y - 1, fill);
    result = (result << 1) + sample_pixel(img, x + 1, y - 1, fill);
    result = (result << 1) + sample_pixel(img, x - 1, y, fill);
    result = (result << 1) + sample_pixel(img, x,  y, fill);
    result = (result << 1) + sample_pixel(img, x + 1, y, fill);
    result = (result << 1) + sample_pixel(img, x - 1, y + 1, fill);
    result = (result << 1) + sample_pixel(img, x,  y + 1, fill);
    result = (result << 1) + sample_pixel(img, x + 1, y + 1, fill);
    result
}

fn apply_filter(img: &[String], filter: &str, fill: usize) -> Vec<String> {

    // prepare output image
    let mut output = vec![String::new(); img.len() + 2];

    for (ny, line) in output.iter_mut().enumerate() {
        let y = ny as i32 - 1;
        for nx in 0..img[0].len() + 2 {
            let x = nx as i32 - 1;
            let idx = filter_index(img, x, y, fill);
            line.push(filter.chars().nth(idx).unwrap());
        }
    }

    output
}

fn count_lit(img: &[String]) -> usize {
    img.iter().map(|s| s.chars().filter(|c| *c == '#').count()).sum()
}


fn main() {
    let mut input_iter = include_str!("../../input.txt").trim_end().lines();

    // read input data
    let filter = input_iter.next().unwrap();
    let mut image: Vec<String> = input_iter.skip(1).map(|l| l.to_string()).collect();

    // some input validation
    assert_eq!(filter.len(), 512);
    assert!((filter.starts_with('.') && filter.ends_with('#')) ||
            (filter.starts_with('#') && filter.ends_with('.')));

    // solve problems
    let filler: [usize; 2] = if filter.starts_with('.') {[0, 0]} else {[0, 1]};

    for step in 0..50 {
        image = apply_filter(&image, filter, filler[step % 2]);
        println!("Pass {} - number of lit pixels = {}", step + 1, count_lit(&image));
    }
}
