fn opposite_polarity(x: char, y: char) -> bool {
    x.to_lowercase().eq(y.to_lowercase()) &&
        ((x.is_uppercase() && y.is_lowercase())
            || (x.is_lowercase() && y.is_uppercase()))
}

fn react(input: &str) -> Vec<char> {
    let chars: Vec<char> = input.trim().chars().collect();
    let mut stack: Vec<char> = vec![];


    for (i, c) in chars.iter().enumerate() {
            if stack.len() > 0 && opposite_polarity(*c, *stack.last().unwrap()) {
                stack.pop();
            } else {
                stack.push(*c);
            }
    }

    return stack;
}

fn part_a(input: &str) -> usize {
    let out =react(input);
    return out.len();
}

fn part_b(input: &str) -> usize {
    let chars: Vec<char> = input.trim().chars().collect();
    let mut set = chars.iter().map(|x| x.to_ascii_lowercase()).collect::<Vec<char>>();
    set.sort();
    set.dedup();

    let mut shortest = chars.len();
    for c in set {
        let mut chars_copy = chars.clone();
        chars_copy.retain(|x| x.to_ascii_lowercase() != c);

        let chars_str: String = chars_copy.iter().collect();
        let current = react(chars_str.as_str());

        if current.len() < shortest {
            shortest = current.len();
        }
    }
    return shortest;
}

fn main() {
    let input = include_str!("input.txt");

    println!("Solution A {}", part_a(input));
    println!("Solution B {}", part_b(input));
}


#[cfg(test)]
mod tests {
    use super::react;
    use super::part_a;
    use super::part_b;

    #[test]
    fn part_a_case_1() {
        assert_eq!("dabCBAcaDA", react("dabAcCaCBAcCcaDA"));
    }


    #[test]
    fn part_a_case_2() {
        assert_eq!(10, part_a("dabAcCaCBAcCcaDA"));
    }

    #[test]
    fn part_b_case_1() {
        assert_eq!("daDA", react("dabAaBAaDA"));
    }

    #[test]
    fn part_b_case_2() {
        assert_eq!(4, part_b("dabAcCaCBAcCcaDA"));
    }
   
}