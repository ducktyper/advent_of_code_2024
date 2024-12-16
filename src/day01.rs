static MAX_INPUT_SIZE: usize = 1000;

pub fn part1(input: &str) -> i32 {
    let mut lefts: [i32; MAX_INPUT_SIZE] = [0; MAX_INPUT_SIZE];
    let mut rights: [i32; MAX_INPUT_SIZE] = [0; MAX_INPUT_SIZE];
    let max_i = load(input, &mut lefts, &mut rights);

    sort(&mut lefts, max_i);
    sort(&mut rights, max_i);

    distance(&lefts, &rights, max_i)
}

pub fn part2(input: &str) -> i32 {
    let mut lefts: [i32; MAX_INPUT_SIZE] = [0; MAX_INPUT_SIZE];
    let mut rights: [i32; MAX_INPUT_SIZE] = [0; MAX_INPUT_SIZE];
    let max_i = load(input, &mut lefts, &mut rights);

    sort(&mut lefts, max_i);
    sort(&mut rights, max_i);

    similarity(&lefts, &rights, max_i)
}

fn distance(lefts: &[i32; MAX_INPUT_SIZE], rights: &[i32; MAX_INPUT_SIZE], max_i: usize) -> i32 {
    let mut output: i32 = 0;
    for i in 0..=max_i {
        let left = lefts[i];
        let right = rights[i];
        if left > right {
            output += left - right;
        } else {
            output += right - left;
        }
    }
    output
}

fn similarity(lefts: &[i32; MAX_INPUT_SIZE], rights: &[i32; MAX_INPUT_SIZE], max_i: usize) -> i32 {
    let mut output = 0;
    let mut left_i = 0;
    let mut right_i = 0;

    loop {
        let left = lefts[left_i];
        let right = rights[right_i];
        if left == right {
            let mut left_dup_count = 1;
            while left_i < max_i && left == lefts[left_i + 1] {
                left_i += 1;
                left_dup_count += 1;
            }

            let mut right_dup_count = 1;
            while right_i < max_i && right == rights[right_i + 1] {
                right_i += 1;
                right_dup_count += 1;
            }

            output += left * left_dup_count * right_dup_count;
            left_i += 1;
            right_i += 1;
        } else if left < right {
            left_i += 1;
        } else if left > right {
            right_i += 1;
        }
        if left_i > max_i || right_i > max_i {
            break;
        }
    }

    output
}

fn load(
    input: &str,
    lefts: &mut [i32; MAX_INPUT_SIZE],
    rights: &mut [i32; MAX_INPUT_SIZE],
) -> usize {
    let max_i: usize = input.lines().count() - 1;
    for (index, line) in input.lines().enumerate() {
        let (left, right) = line.split_once("   ").unwrap();
        lefts[index] = left.parse::<i32>().unwrap();
        rights[index] = right.parse::<i32>().unwrap();
    }
    max_i
}

fn sort(input: &mut [i32; MAX_INPUT_SIZE], max_index: usize) {
    let mut index: usize = 0;
    while index < max_index {
        let current = input[index];
        let next = input[index + 1];
        if current <= next {
            index += 1;
        } else {
            input[index] = next;
            input[index + 1] = current;
            if index > 0 {
                index -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn part1_test() {
        let data = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
        assert_eq!(part1(&data), 11);
    }

    #[test]
    fn part2_test() {
        let data = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
        assert_eq!(part2(&data), 31);
    }
}
