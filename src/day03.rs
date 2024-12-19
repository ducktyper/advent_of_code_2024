pub fn part1(input: &str) -> i32 {
    let mut output = 0;

    let format = ['m', 'u', 'l', '(', 'A', ',', 'B', ')'];
    let mut format_index = 0;
    let mut a = 0;
    let mut b = 0;

    for c in input.chars() {
        if c == format[0] {
            format_index = 1;
            continue;
        }

        if format[format_index] == 'A' {
            if c.is_digit(10) {
                let number = c as i32 - '0' as i32;
                a = a * 10 + number;
                continue;
            } else {
                format_index += 1;
            }
        }

        if format[format_index] == 'B' {
            if c.is_digit(10) {
                let number = c as i32 - '0' as i32;
                b = b * 10 + number;
                continue;
            } else {
                format_index += 1;
            }
        }

        if "mul(,".contains(format[format_index]) {
            if c == format[format_index] {
                format_index += 1;
                continue;
            }
        }

        if format[format_index] == ')' && c == format[format_index] {
            output += a * b;
        }

        // reset
        format_index = 0;
        a = 0;
        b = 0;
    }
    output
}

pub fn part2(input: &str) -> i32 {
    let mut output = 0;

    let mut enabled = true;
    let enable = ['d', 'o', '(', ')'];
    let mut enable_index = 0;
    let disable = ['d', 'o', 'n', '\'', 't', '(', ')'];
    let mut disable_index = 0;

    let format = ['m', 'u', 'l', '(', 'A', ',', 'B', ')'];
    let mut format_index = 0;
    let mut a = 0;
    let mut b = 0;

    for c in input.chars() {
        // enable
        if c == enable[0] {
            enable_index = 1;
        } else if c == enable[enable_index] {
            if c == ')' {
                enabled = true;
                enable_index = 0;
            } else {
                enable_index += 1;
            }
        } else {
            enable_index = 0;
        }

        // disable
        if c == disable[0] {
            disable_index = 1;
        } else if c == disable[disable_index] {
            if c == ')' {
                enabled = false;
                disable_index = 0;
            } else {
                disable_index += 1;
            }
        } else {
            disable_index = 0;
        }

        // format
        if c == format[0] {
            format_index = 1;
            continue;
        }

        if format[format_index] == 'A' {
            if c.is_digit(10) {
                let number = c as i32 - '0' as i32;
                a = a * 10 + number;
                continue;
            } else {
                format_index += 1;
            }
        }

        if format[format_index] == 'B' {
            if c.is_digit(10) {
                let number = c as i32 - '0' as i32;
                b = b * 10 + number;
                continue;
            } else {
                format_index += 1;
            }
        }

        if "mul(,".contains(format[format_index]) {
            if c == format[format_index] {
                format_index += 1;
                continue;
            }
        }

        if format[format_index] == ')' && c == format[format_index] {
            if enabled {
                output += a * b;
            }
        }

        // reset
        format_index = 0;
        a = 0;
        b = 0;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn part1_test() {
        let data = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        assert_eq!(part1(&data), 161);
    }

    #[test]
    fn part2_test() {
        let data = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        assert_eq!(part2(&data), 48);
    }
}
