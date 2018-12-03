use std::collections::{HashSet, HashMap};

// TODO(refactor into a single function)

fn part_a(input: &str) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut overlapping: HashSet<(usize, usize)> = HashSet::new();

    for l in input.lines() {
        let v = l.split(|c| c == ' ' || c == '@' || c == ',' || c == ':' || c == 'x' || c == '#')
            .filter_map(|c| c.parse::<usize>().ok())
            .collect::<Vec<_>>();
        let (_, from_left, from_top, w, h) = (v[0], v[1], v[2], v[3], v[4]);

        for i in from_left..from_left+w {
            for j in from_top..from_top+h {
                let new: bool = visited.insert((i, j));
                if !new {
                    overlapping.insert((i,j));
                }
            }
        }
    }

    return overlapping.len();
}

fn part_b(input: &str) -> usize {
    let mut last_seen_at: HashMap<(usize, usize), usize> = HashMap::new();
    let mut visited: HashSet<usize> = HashSet::new();
    let mut intersecting: HashSet<usize> = HashSet::new();
    
    for l in input.lines() {
        let v = l.split(|c| c == ' ' || c == '@' || c == ',' || c == ':' || c == 'x' || c == '#')
            .filter_map(|c| c.parse::<usize>().ok())
            .collect::<Vec<_>>();
        let (id, from_left, from_top, w, h) = (v[0], v[1], v[2], v[3], v[4]);
        visited.insert(id);
        for i in from_left..from_left+w {
            for j in from_top..from_top+h {
                let new: bool = last_seen_at.contains_key(&(i,j));
                if !new {
                    last_seen_at.insert((i,j), id);
                } else {
                    intersecting.insert(last_seen_at[&(i,j)]);
                    intersecting.insert(id);
                }
            }
        }
    }

    let not_overlapping_id: usize = *visited.difference(&intersecting).next().expect("Value not found");

    return not_overlapping_id;
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part_a(input));
    println!("{}", part_b(input));
}


#[cfg(test)]
mod tests {
    use super::part_a;
    use super::part_b;
    
    #[test]
    fn part_a_case_1() {
        assert_eq!(4, part_a("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"));
    }

    #[test]
    fn part_b_case_1() {
        assert_eq!(3, part_b("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"));
    }
}