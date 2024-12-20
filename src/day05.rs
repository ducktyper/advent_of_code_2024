pub fn part1(input: &str) -> i32 {
    let mut output = 0;

    let mut rules: [Vec<usize>; 100] = std::array::from_fn(|_| Vec::new());
    let mut reading_rules = true;

    for line in input.lines() {
        if line == "" {
            reading_rules = false;
            continue;
        }
        if reading_rules {
            let values: Vec<usize> = line
                .split('|')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            rules[values[1]].push(values[0]);
        } else {
            let mut numbers: [i32; 100] = [0; 100]; // 1: used somewhere, 2: used before
            let mut pass = true;
            let values: Vec<usize> = line
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            for value in &values {
                numbers[*value] = 1;
            }
            for value in &values {
                numbers[*value] = 2;
                for expected_value in &rules[*value] {
                    if numbers[*expected_value] == 1 {
                        pass = false;
                        break;
                    }
                }
            }
            if pass {
                output += values[values.len() / 2] as i32;
            }
        }
    }

    output
}

pub fn part2(input: &str) -> i32 {
    let mut output = 0;

    let mut rules: [Vec<usize>; 100] = std::array::from_fn(|_| Vec::new());
    let mut reading_rules = true;

    for line in input.lines() {
        if line == "" {
            reading_rules = false;
            continue;
        }
        if reading_rules {
            let values: Vec<usize> = line
                .split('|')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            rules[values[1]].push(values[0]);
        } else {
            let mut numbers: [i32; 100] = [0; 100]; // 1: used somewhere, 2: used before
            let mut pass = true;
            let mut values: Vec<usize> = line
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            for value in &values {
                numbers[*value] = 1;
            }
            let mut index = 0;
            while index < values.len() {
                let mut fixed = false;
                for expected_value in &rules[values[index]] {
                    if numbers[*expected_value] == 1 {
                        let element = values.remove(index);
                        values.push(element);
                        fixed = true;
                        pass = false;
                        break;
                    }
                }
                if fixed {
                    continue;
                }
                numbers[values[index]] = 2;
                index += 1;
            }
            if !pass {
                output += values[values.len() / 2] as i32;
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn part1_test() {
        let data = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;
        assert_eq!(part1(&data), 143);
    }

    #[test]
    fn part2_test() {
        let data = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;
        assert_eq!(part2(&data), 123);
    }
}
