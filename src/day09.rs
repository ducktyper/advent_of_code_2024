pub fn part1(input: &str) -> i64 {
    let mut output = 0;

    let mut numbers: Vec<i64> = Vec::new();
    let mut index = 0;
    for c in input.chars() {
        let number = c as i64 - '0' as i64;
        let value = if index % 2 == 0 { index / 2 } else { -1 };
        for _ in 0..number {
            numbers.push(value);
        }
        index += 1;
    }

    let mut i_a = 0;
    let mut i_b = numbers.len() - 1;
    while i_a < i_b {
        if numbers[i_b] == -1 {
            i_b -= 1;
            continue;
        }
        if numbers[i_a] == -1 {
            numbers[i_a] = numbers[i_b];
            i_b -= 1;
        }
        i_a += 1;
    }

    for i in 0..=i_b {
        output += numbers[i] * i as i64;
    }

    output
}

#[derive(Debug)]
struct Blank {
    index: usize,
    count: usize,
}

#[derive(Debug)]
struct Filled {
    index: usize,
    count: usize,
    value: i64,
}

pub fn part2(input: &str) -> i64 {
    let mut output = 0;

    let mut numbers: Vec<i64> = Vec::new();
    let mut index = 0;
    for c in input.chars() {
        let number = c as i64 - '0' as i64;
        let value = if index % 2 == 0 { index / 2 } else { -1 };
        for _ in 0..number {
            numbers.push(value);
        }
        index += 1;
    }

    let mut blanks: Vec<Blank> = Vec::new();
    let mut filled: Vec<Filled> = Vec::new();
    let mut batch_value = numbers[0].clone();
    let mut batch_index = 0;
    for (index, value) in numbers.iter().enumerate() {
        if *value == batch_value {
            continue;
        } else {
            if batch_value == -1 {
                blanks.push(Blank {
                    index: batch_index,
                    count: index - batch_index,
                });
            } else {
                filled.push(Filled {
                    index: batch_index,
                    count: index - batch_index,
                    value: batch_value,
                });
            }
            batch_value = value.clone();
            batch_index = index;
        }
    }

    if batch_value == -1 {
        blanks.push(Blank {
            index: batch_index,
            count: numbers.len() - batch_index,
        });
    } else {
        filled.push(Filled {
            index: batch_index,
            count: numbers.len() - batch_index,
            value: batch_value,
        });
    }

    for i in (0..filled.len()).rev() {
        let fill = &filled[i];
        for j in 0..blanks.len() {
            let blank = blanks.get_mut(j).unwrap();
            if blank.index > fill.index {
                break;
            }
            if blank.count >= fill.count {
                for z in blank.index..(blank.index + fill.count) {
                    numbers[z] = fill.value;
                }
                for z in fill.index..(fill.index + fill.count) {
                    numbers[z] = -1;
                }
                blank.count -= fill.count;
                blank.index += fill.count;
                break;
            }
        }
    }

    for i in 0..numbers.len() {
        if numbers[i] == -1 {
            continue;
        }
        output += numbers[i] * i as i64;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn part1_test() {
        let data = "2333133121414131402";
        assert_eq!(part1(&data), 1928);
    }

    #[test]
    fn part2_test() {
        let data = "2333133121414131402";
        assert_eq!(part2(&data), 2858);
    }
}
