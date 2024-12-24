pub fn part1(input: &str) -> i32 {
    let mut output = 0;

    let mut data = [[-1; 132]; 132]; // -1: outside, 0: used, 1: wall, 2: visited
    let mut i_current: usize = 0;
    let mut j_current: usize = 0;
    let directions: [[i32; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];
    let mut direction_index = 0;
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                data[i + 1][j + 1] = 0;
            } else {
                data[i + 1][j + 1] = 1;
            }
            if c == '^' {
                i_current = i + 1;
                j_current = j + 1;
            }
        }
    }

    while data[i_current][j_current] != -1 {
        data[i_current][j_current] = 2;
        if data[(i_current as i32 + directions[direction_index][0]) as usize]
            [(j_current as i32 + directions[direction_index][1]) as usize]
            == 0
        {
            direction_index = (direction_index + 1) % 4;
        }
        i_current = (i_current as i32 + directions[direction_index][0]) as usize;
        j_current = (j_current as i32 + directions[direction_index][1]) as usize;
    }

    for row in data {
        for value in row {
            if value == 2 {
                output += 1;
            }
        }
    }

    output
}

const DIRECTIONS: [[i32; 3]; 4] = [[-1, 0, 2], [0, 1, 3], [1, 0, 5], [0, -1, 7]];

pub fn part2(input: &str) -> i32 {
    let mut output = 0;

    let mut data = [[-1; 132]; 132]; // -1: outside, 0: wall, 1: used, >1: visited
    let mut i_current: usize = 0;
    let mut j_current: usize = 0;
    let mut direction_index = 0;
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                data[i + 1][j + 1] = 0;
            } else {
                data[i + 1][j + 1] = 1;
            }
            if c == '^' {
                i_current = i + 1;
                j_current = j + 1;
            }
        }
    }

    while data[i_current][j_current] != -1 {
        output += wall(i_current, j_current, direction_index, data);
        data[i_current][j_current] *= DIRECTIONS[direction_index][2];
        if data[(i_current as i32 + DIRECTIONS[direction_index][0]) as usize]
            [(j_current as i32 + DIRECTIONS[direction_index][1]) as usize]
            == 0
        {
            direction_index = (direction_index + 1) % 4;
        }
        i_current = (i_current as i32 + DIRECTIONS[direction_index][0]) as usize;
        j_current = (j_current as i32 + DIRECTIONS[direction_index][1]) as usize;
    }

    output
}

fn wall(
    mut i_current: usize,
    mut j_current: usize,
    mut direction_index: usize,
    mut data: [[i32; 132]; 132],
) -> i32 {
    while data[(i_current as i32 + DIRECTIONS[direction_index][0]) as usize]
        [(j_current as i32 + DIRECTIONS[direction_index][1]) as usize]
        == 0
    {
        direction_index = (direction_index + 1) % 4;
    }
    let i_current_wall = (i_current as i32 + DIRECTIONS[direction_index][0]) as usize;
    let j_current_wall = (j_current as i32 + DIRECTIONS[direction_index][1]) as usize;
    if data[i_current_wall][j_current_wall] > 1 {
        return 0;
    }
    data[i_current_wall][j_current_wall] = 0;

    while data[i_current][j_current] != -1 {
        if data[i_current][j_current] % DIRECTIONS[direction_index][2] == 0 {
            return 1;
        }
        data[i_current][j_current] *= DIRECTIONS[direction_index][2];
        while data[(i_current as i32 + DIRECTIONS[direction_index][0]) as usize]
            [(j_current as i32 + DIRECTIONS[direction_index][1]) as usize]
            == 0
        {
            direction_index = (direction_index + 1) % 4;
        }
        i_current = (i_current as i32 + DIRECTIONS[direction_index][0]) as usize;
        j_current = (j_current as i32 + DIRECTIONS[direction_index][1]) as usize;
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn part1_test() {
        let data = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;
        assert_eq!(part1(&data), 41);
    }

    #[test]
    fn part2_test() {
        let data = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;
        assert_eq!(part2(&data), 6);
    }
}
