use std::collections::{HashMap, HashSet};

struct Bound {
    max_rows: i32,
    max_cols: i32,
    min_rows: i32,
    min_cols: i32
}

fn abs(x: i32) -> i32 {
    let y = if x >= 0 { x } else { -x };
    y
}

fn manhattan(x: (i32, i32), y: (i32, i32)) -> i32 {
    let a: i32 = (x.0 - y.0);
    let b: i32 = (x.1 - y.1);
    return abs(a) + abs(b);
}

fn parse_input(input :&str) -> (HashMap<(i32, i32), i32>, HashSet<(i32, i32)>, Bound) {
    let mut points: HashMap<(i32, i32), i32> = HashMap::new();
    let mut infinite: HashSet<(i32, i32)> = HashSet::<(i32, i32)>::new();

    let mut max_rows: i32 = 0;
    let mut max_cols: i32 = 0;
    let mut min_rows: i32 = 0;
    let mut min_cols: i32 = 0;

    let mut bound = Bound {
        max_rows:  0,
        max_cols: 0,
        min_rows: 0,
        min_cols: 0
    };

    for (i, l) in input.lines().enumerate() {
        let v: Vec<i32> = l.trim()
            .split(", ").map(|x: &str| x.parse::<i32>()
            .expect("Error parsing number"))
            .collect::<Vec<i32>>();
        let (x, y) = (v[0], v[1]);

        bound.max_rows = bound.max_rows.max(x);
        bound.max_cols = bound.max_cols.max(y);
        bound.min_rows = bound.min_rows.min(x);
        bound.min_cols = bound.min_cols.min(y);

        points.insert((x, y), 0);
    }

    for i in bound.min_rows..=bound.max_rows {
        for j in bound.min_cols..=bound.max_cols {
            let mut min_distance = std::i32::MAX;
            let mut closer = (bound.min_rows, bound.min_cols);
            for (point, _) in &points {
                let distance = manhattan( (i, j), *point);
                if distance < min_distance {
                    min_distance = distance;
                    closer = *point;
                } else if distance == min_distance {
                    closer = (-1, -1)
                }
            }

            if closer != (-1, -1) {
                *points.entry(closer).or_insert(0) += 1;
                if (i <= bound.min_rows || i >= bound.max_rows) ||
                    (j <= bound.min_cols  || j >= bound.max_cols) {
                    infinite.insert(closer); // points that live in an infinite area
                }
            }
        }
    }
    
    return (points, infinite, bound);
}

fn part_a(input: &str) -> i32 {

    let (mut points,
        infinite,
        _) = parse_input(input);

    points.retain(|point, _|
        !infinite.contains(point));

    let max_area: i32 = *points.values().max().unwrap();
    
    return max_area;
}

fn part_b(input: &str) -> usize {
    const MAX_DISTANCE: i32 = 10_000;
    
    let (mut points, _, bound) = parse_input(input);
    let mut region = 0;

    for i in bound.min_rows..=bound.max_rows {
        for j in bound.min_cols..=bound.max_cols {
            let mut distance = 0;
            for &point in points.keys() {
                distance += manhattan((i, j), point);
            }

            if distance < MAX_DISTANCE {
                region += 1;
            }
        }
    }
    return region;
}

fn main() {
    let input = include_str!("input.txt");

    println!("Solution A {}", part_a(input));
    println!("Solution B {}", part_b(input));
}


#[cfg(test)]
mod tests {
    use super::part_a;
    use super::part_b;

    #[test]
    fn part_a_case_1() {
        assert_eq!(17, part_a("1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
"))
    }

}
