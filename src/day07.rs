pub fn part1(input: &str) -> i64 {
    let mut output = 0;

    for line in input.lines() {
        let mut target = 0;
        let mut target_done = false;
        let mut numbers: Vec<i64> = Vec::new();
        let mut numbers_count: usize = 0;
        for c in line.chars() {
            if c == ':' {
                target_done = true;
                continue;
            }
            if c == ' ' {
                numbers.push(0);
                numbers_count += 1;
                continue;
            }
            let number = c as i64 - '0' as i64;
            if !target_done {
                target = target * 10 + number;
                continue;
            }
            numbers[numbers_count - 1] = numbers[numbers_count - 1] * 10 + number;
        }
        if possible(target, &numbers) {
            output += target;
        }
    }

    output
}

fn possible(target: i64, numbers: &Vec<i64>) -> bool {
    let max_iteration: i64 = (2 as i64).pow((numbers.len() - 1) as u32);
    for i in 0..max_iteration {
        let mut sum = numbers[0];
        for (j, val) in numbers.iter().enumerate() {
            if j == 0 {
                continue;
            }
            if (i / (2 as i64).pow((j - 1) as u32)) % 2 == 0 {
                sum += val;
            } else {
                sum *= val;
            }
        }
        if sum == target {
            return true;
        }
    }
    false
}

pub fn part2(input: &str) -> i64 {
    let mut output = 0;

    for line in input.lines() {
        let mut target = 0;
        let mut target_done = false;
        let mut numbers: Vec<i64> = Vec::new();
        let mut numbers_count: usize = 0;
        for c in line.chars() {
            if c == ':' {
                target_done = true;
                continue;
            }
            if c == ' ' {
                numbers.push(0);
                numbers_count += 1;
                continue;
            }
            let number = c as i64 - '0' as i64;
            if !target_done {
                target = target * 10 + number;
                continue;
            }
            numbers[numbers_count - 1] = numbers[numbers_count - 1] * 10 + number;
        }
        if possible_with_pipe(target, &numbers) {
            output += target;
        }
    }

    output
}

fn possible_with_pipe(target: i64, numbers: &Vec<i64>) -> bool {
    let max_iteration: i64 = (3 as i64).pow((numbers.len() - 1) as u32);
    for i in 0..max_iteration {
        let mut sum = numbers[0];
        for (j, val) in numbers.iter().enumerate() {
            if j == 0 {
                continue;
            }
            if (i / (3 as i64).pow((j - 1) as u32)) % 3 == 0 {
                sum += val;
            } else if (i / (3 as i64).pow((j - 1) as u32)) % 3 == 1 {
                sum *= val;
            } else {
                sum = sum * (10 as i64).pow(val.to_string().len() as u32) + val;
            }
        }
        if sum == target {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn part1_test() {
        let data = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;
        assert_eq!(part1(&data), 3749);
    }

    #[test]
    fn part2_test() {
        let data = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;
        assert_eq!(part2(&data), 11387);
    }
}
