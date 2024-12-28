use std::collections::HashMap;

#[derive(Debug)]
struct Point {
    i: usize,
    j: usize,
}

pub fn part1(input: &str) -> i32 {
    let mut output = 0;

    let mut data = [[' '; 50]; 50];
    let mut data_filled = [[' '; 50]; 50];
    let max_i = fill_data(input, &mut data);
    let mut map: HashMap<char, Vec<Point>> = HashMap::new();

    for i in 0..=max_i {
        for j in 0..=max_i {
            if data[i][j] == '.' || data[i][j] == '#' {
                continue;
            }
            let points = map.entry(data[i][j]).or_insert_with(Vec::new);
            let new_point = Point { i, j };
            for point in points.iter() {
                let a_i = point.i as i32 + point.i as i32 - new_point.i as i32;
                let a_j = point.j as i32 + point.j as i32 - new_point.j as i32;
                if a_i >= 0 && a_i <= max_i as i32 && a_j >= 0 && a_j <= max_i as i32 {
                    data_filled[a_i as usize][a_j as usize] = '#';
                }
                let b_i = new_point.i as i32 + new_point.i as i32 - point.i as i32;
                let b_j = new_point.j as i32 + new_point.j as i32 - point.j as i32;
                if b_i >= 0 && b_i <= max_i as i32 && b_j >= 0 && b_j <= max_i as i32 {
                    data_filled[b_i as usize][b_j as usize] = '#';
                }
            }
            points.push(new_point);
        }
    }

    for i in 0..=max_i {
        for j in 0..=max_i {
            if data_filled[i][j] == '#' {
                output += 1;
            }
        }
    }

    output
}

pub fn part2(input: &str) -> i32 {
    let mut output = 0;

    let mut data = [[' '; 50]; 50];
    let mut data_filled = [[' '; 50]; 50];
    let max_i = fill_data(input, &mut data);
    let mut map: HashMap<char, Vec<Point>> = HashMap::new();

    for i in 0..=max_i {
        for j in 0..=max_i {
            if data[i][j] == '.' || data[i][j] == '#' {
                continue;
            }
            data_filled[i][j] = '#';
            let points = map.entry(data[i][j]).or_insert_with(Vec::new);
            let new_point = Point { i, j };
            for point in points.iter() {
                let mut a_count = 1;
                loop {
                    let a_i = point.i as i32 + (point.i as i32 - new_point.i as i32) * a_count;
                    let a_j = point.j as i32 + (point.j as i32 - new_point.j as i32) * a_count;
                    if a_i >= 0 && a_i <= max_i as i32 && a_j >= 0 && a_j <= max_i as i32 {
                        data_filled[a_i as usize][a_j as usize] = '#';
                        a_count += 1;
                    } else {
                        break;
                    }
                }
                let mut b_count = 1;
                loop {
                    let b_i = new_point.i as i32 + (new_point.i as i32 - point.i as i32) * b_count;
                    let b_j = new_point.j as i32 + (new_point.j as i32 - point.j as i32) * b_count;
                    if b_i >= 0 && b_i <= max_i as i32 && b_j >= 0 && b_j <= max_i as i32 {
                        data_filled[b_i as usize][b_j as usize] = '#';
                        b_count += 1;
                    } else {
                        break;
                    }
                }
            }
            points.push(new_point);
        }
    }

    for i in 0..=max_i {
        for j in 0..=max_i {
            if data_filled[i][j] == '#' {
                output += 1;
            }
        }
    }

    output
}

fn fill_data(input: &str, data: &mut [[char; 50]; 50]) -> usize {
    let mut i = 0;
    let mut j;
    for line in input.lines() {
        j = 0;
        for c in line.chars() {
            data[i][j] = c;
            j += 1;
        }
        i += 1;
    }
    i - 1
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn part1_test() {
        let data = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;
        assert_eq!(part1(&data), 14);
    }

    #[test]
    fn part2_test() {
        let data = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;
        assert_eq!(part2(&data), 34);
    }
}
