use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::io::Error;

fn parse_input(fname: String) -> Vec<i32> {
    let f: File = File::open(fname).expect("Unable to open file");
    let f: BufReader<File> = BufReader::new(f);

    let input: Vec<i32> = f.lines()
        .map(|l: Result<String, Error>| {
            l.unwrap().parse::<i32>().expect("Not a number")
        }).collect();

    return input;
}

fn part_a(input: Vec<i32>) -> i32 {
    return input.iter().sum()
}

fn part_b(input: Vec<i32>) -> i32 {
    let mut frequencies: HashSet<i32> = HashSet::new();
    let mut frequency: i32 = 0;

    frequencies.insert(frequency);
    for number in input.iter().cycle() {
        frequency += number;
        if frequencies.contains(&frequency) {
            break
        }
        frequencies.insert(frequency);
    }

    return frequency
}

fn main() {
    println!("Solution A: {}", part_a(parse_input("input.txt".to_string())));
    println!("Solution B: {}", part_b(parse_input("input.txt".to_string())));
}


#[cfg(test)]
mod tests {
    use super::part_a;
    use super::part_b;

    #[test]
    fn part_a_case_1() {
        assert_eq!(3, part_a(vec![1, 1, 1]));
    }

    #[test]
    fn part_a_case_2() {
        assert_eq!(0, part_a(vec![1, 1, -2]));
    }

    #[test]
    fn part_a_case_3() {
        assert_eq!(-6, part_a(vec![-1, -2, -3]));
    }

    #[test]
    fn part_b_case_1() {
        assert_eq!(0, part_b(vec![1, -1]));
    }

    #[test]
    fn part_b_case_2() {
        assert_eq!(10, part_b(vec![3, 3, 4, -2, -4]));
    }

    #[test]
    fn part_b_case_3() {
        assert_eq!(5, part_b(vec![-6, 3, 8, 5, -6]));
    }

    #[test]
    fn part_b_case_4() {
        assert_eq!(14, part_b(vec![7, 7, -2, -7, -4]));
    }
}