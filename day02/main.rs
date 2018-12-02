extern crate counter;

use counter::Counter;

fn part_a(input: &str) -> u32 {
    let mut cnt_twice: u32 = 0;
    let mut cnt_thrice: u32 = 0;

    for l in input.lines() {
        let char_counts = l.trim().chars().collect::<Counter<_>>();

        let mut found_twice = false;
        let mut found_thrice = false;
        for i in char_counts.values() {
            if *i as u32 == 2  && !found_twice {
                cnt_twice += 1;
                found_twice = true
            }
            if *i as u32 == 3 && !found_thrice {
                cnt_thrice += 1;
                found_thrice = true;
            }
        }
    }
    return cnt_twice * cnt_thrice;
}

fn part_b(input: &str) -> String {
    for l1 in input.lines() {
        for l2 in input.lines() {
            let mut diffs: u32 = 0;
            let mut common_chars: Vec<char> = vec![];
            for chars  in l1.chars().zip(l2.chars()) {
                if chars.0 != chars.1 {
                    diffs += 1;
                } else {
                    common_chars.push(chars.0);
                }
                if diffs > 1 {
                    break
                }
            }
            if diffs == 1 {
                return common_chars.into_iter().collect();
            }
        }
    }

    return String::from("");
}

fn main() {
    let input: &str = include_str!("input.txt");
    println!("Solution A: {}", part_a(input));
    println!("Solution B: {}", part_b(input));
}


#[cfg(test)]
mod tests {
    use super::part_a;
    use super::part_b;

    #[test]
    fn part_a_case_1() {
        assert_eq!(0, part_a("abcdef"));
    }

    #[test]
    fn part_a_case_2() {
        assert_eq!(1, part_a("bababc"));
    }
    
    #[test]
    fn part_a_case_3() {
        assert_eq!(0, part_a("abbcde"));
    }

    #[test]
    fn part_a_case_4() {
        assert_eq!(0, part_a("ababab"));
    }

    #[test]
    fn part_b_case_1() {
        assert_eq!("fgij", part_b("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"));
    }


}
