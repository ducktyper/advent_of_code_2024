const ISTART: usize = 3;
const JSTART: usize = 3;

pub fn part1(input: &str) -> i32 {
    let mut output = 0;

    let mut data = [['O'; 150]; 150];

    let (max_i, max_j) = fill_data(input, &mut data);

    for i in ISTART..=max_i {
        for j in JSTART..=max_j {
            output += find_xmas(&data, i, j, 0, 1);
            output += find_xmas(&data, i, j, 0, -1);
            output += find_xmas(&data, i, j, 1, 0);
            output += find_xmas(&data, i, j, -1, 0);
            output += find_xmas(&data, i, j, 1, 1);
            output += find_xmas(&data, i, j, 1, -1);
            output += find_xmas(&data, i, j, -1, 1);
            output += find_xmas(&data, i, j, -1, -1);
        }
    }

    output
}

pub fn part2(input: &str) -> i32 {
    let mut output = 0;

    let mut data = [['O'; 150]; 150];

    let (max_i, max_j) = fill_data(input, &mut data);

    for i in ISTART..=max_i {
        for j in JSTART..=max_j {
            output += find_x_mas(&data, i, j);
        }
    }

    output
}

fn find_xmas(
    data: &[[char; 150]; 150],
    i: usize,
    j: usize,
    i_direction: i32,
    j_direction: i32,
) -> i32 {
    let matched = data[i][j] == 'X'
        && data[(i as i32 + i_direction) as usize][(j as i32 + j_direction) as usize] == 'M'
        && data[(i as i32 + i_direction * 2) as usize][(j as i32 + j_direction * 2) as usize]
            == 'A'
        && data[(i as i32 + i_direction * 3) as usize][(j as i32 + j_direction * 3) as usize]
            == 'S';

    if matched {
        1
    } else {
        0
    }
}

fn find_x_mas(data: &[[char; 150]; 150], i: usize, j: usize) -> i32 {
    let one_side = data[i][j] == 'A'
        && ((data[i + 1][j + 1] == 'M' && data[i - 1][j - 1] == 'S')
            || (data[i + 1][j + 1] == 'S' && data[i - 1][j - 1] == 'M'));
    let other_side = data[i][j] == 'A'
        && ((data[i + 1][j - 1] == 'M' && data[i - 1][j + 1] == 'S')
            || (data[i + 1][j - 1] == 'S' && data[i - 1][j + 1] == 'M'));

    if one_side && other_side {
        1
    } else {
        0
    }
}

fn fill_data(input: &str, data: &mut [[char; 150]; 150]) -> (usize, usize) {
    let mut i = ISTART;
    let mut j = JSTART;
    for line in input.lines() {
        j = JSTART;
        for c in line.chars() {
            data[i][j] = c;
            j += 1;
        }
        i += 1;
    }
    (i - 1, j - 1)
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn part1_test() {
        let data = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;
        assert_eq!(part1(&data), 18);
    }

    #[test]
    fn part2_test() {
        let data = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;
        assert_eq!(part2(&data), 9);
    }
}
