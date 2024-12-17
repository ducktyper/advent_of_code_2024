pub fn part1(input: &str) -> i32 {
    let mut output = 0;
    for line in input.lines() {
        if safe(line_to_numbers(line)) {
            output += 1;
        }
    }
    output
}

pub fn part2(input: &str) -> i32 {
    let mut output = 0;
    for line in input.lines() {
        let numbers = line_to_numbers(line);
        for i in 0..numbers.len() {
            let sub_numbers = numbers_excluding(numbers.clone(), i);
            if safe(sub_numbers) {
                output += 1;
                break;
            }
        }
    }
    output
}

fn line_to_numbers(line: &str) -> Vec<i32> {
    line.split(' ')
        .map(|x| x.parse::<i32>())
        .filter_map(Result::ok)
        .collect::<Vec<i32>>()
}

fn numbers_excluding(mut numbers: Vec<i32>, index: usize) -> Vec<i32> {
    numbers.remove(index);
    numbers
}

fn safe(numbers: Vec<i32>) -> bool {
    if numbers[0] == numbers[1] {
        return false;
    }
    let direction = numbers[0] < numbers[1];
    for i in 0..(numbers.len() - 1) {
        if numbers[i] - numbers[i + 1] > 3 || numbers[i] - numbers[i + 1] < -3 {
            return false;
        }
        if numbers[i] == numbers[i + 1] {
            return false;
        }
        if direction != (numbers[i] < numbers[i + 1]) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn part1_test() {
        let data = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;
        assert_eq!(part1(&data), 2);
    }

    #[test]
    fn part2_test() {
        let data = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;
        assert_eq!(part2(&data), 4);
    }
}
