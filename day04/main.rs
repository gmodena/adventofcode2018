extern crate regex;

use std::collections::HashMap;
use regex::Regex;

fn parse_shifts(input: &str) -> HashMap<usize, Vec<usize>> {
    let mut guards: HashMap<usize, Vec<usize>> = HashMap::new();

    let re_date: Regex = Regex::new(r"(\d+)-(\d+)-(\d+) (\d+):(\d+)] [(\w?\s)|#(\d+)]+").unwrap();
    let re_guard: Regex = Regex::new(r"#(\d+)").unwrap();

    let mut falls_asleep_at: usize =0 ;
    let mut guard= 0;

    let mut t: Vec<&str> = input.lines().collect();
    t.sort();

    for l in t  {
        let date = re_date.captures(l).unwrap();
        let min_of_hour: usize = date[5].parse::<usize>().unwrap();


        if l.contains("Guard") {
            // shift begins
            guard = re_guard.captures(l).unwrap()[1].parse::<usize>().expect("Failed when parsing guard id");
            let mut minutes: Vec<usize> = vec![0; 60];
            guards.entry(guard).or_insert( minutes);
        }

        if l.contains("asleep") {
            // guard in shift falls asleep at min_of_hour
            falls_asleep_at = min_of_hour;
        }

        if l.contains("wakes") {
            // guard in shift was asleep, wakes up at min_of_hour
            let mut wakes_up_at = min_of_hour;

            if let Some(minutes) = guards.get_mut(&guard) {
                for i in falls_asleep_at..wakes_up_at {
                    minutes[i] += 1;
                }
            }
        }
    }

    guards
}

fn part_a(input: &str) -> usize {
    let mut guards = parse_shifts(input);

    let mut longest_asleep= 0;
    let mut guard_id = 0;
    guards.retain(|guard, minutes| {
        let mut sum: usize = 0;
        for m in minutes {
            sum += *m;
        }
        if sum > longest_asleep {
            longest_asleep = sum;
            guard_id = *guard;
        }
        true
    });

    let minutes = guards.get(&guard_id).unwrap();
    let mut most_minutes: usize = 0;
    let mut minute = 0;
    for (i, value) in minutes.iter().enumerate() {
        if value > &most_minutes {
            most_minutes = *value;
            minute = i;
        }
    }

    return minute * guard_id;
}

fn part_b(input: &str) -> usize {
    let mut guards = parse_shifts(input);

    let mut longest_asleep= 0;
    let mut guard_id: usize = 0;
    let mut most_freq_minute = 0;
    guards.retain(|guard, minutes| {
        let mut curr_longest_asleep: usize = 0;
        let mut index = 0;
        for (i, value) in minutes.iter().enumerate() {
            if value > &curr_longest_asleep {
                curr_longest_asleep = *value;
                index = i;
            }
        }

        if curr_longest_asleep > longest_asleep {
            guard_id = *guard;
            most_freq_minute = index;
            longest_asleep = curr_longest_asleep;
        }

        true
    });

    return most_freq_minute * guard_id;
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
        assert_eq!(240, part_a("[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"));
    }
    
    #[test]
    fn part_b_case_1() {
        assert_eq!(4455, part_b("[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"));
    }
}