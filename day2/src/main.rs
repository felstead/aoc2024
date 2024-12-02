use std::fs::read_to_string;

fn main() {
    println!("Advent of code 2024 - day 2!");

    let levels = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(split_line_to_levels)
        .collect::<Vec<_>>();

    let num_safe = levels
        .iter()
        .fold(0, |acc, l| acc + if is_safe(l) { 1 } else { 0 });

    println!("Num safe (part 1): {}", num_safe);

    let num_safe_with_dampener = levels.iter().fold(0, |acc, l| {
        acc + if is_safe_with_dampener_naive(l) { 1 } else { 0 }
    });

    println!("Num safe (part 2): {}", num_safe_with_dampener);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Asc,
    Desc,
    None,
}

fn get_direction_and_diff(a: i32, b: i32) -> (Direction, i32) {
    let diff = a.abs_diff(b);
    if a > b {
        (Direction::Desc, diff as i32)
    } else if a < b {
        (Direction::Asc, diff as i32)
    } else {
        (Direction::None, 0)
    }
}

fn is_safe_transition(a: i32, b: i32, expected_direction: Option<Direction>) -> (bool, Direction) {
    let (direction, diff) = get_direction_and_diff(a, b);

    let is_safe = match direction {
        Direction::None => false,
        Direction::Asc | Direction::Desc => {
            (expected_direction.is_none() || direction == expected_direction.unwrap()) && diff <= 3
        }
    };

    //println!("{} -> {} ({:?}) : {}", a, b, direction, if is_safe { "safe"} else { "unsafe"} );

    (is_safe, direction)
}

fn split_line_to_levels(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn is_safe_with_dampener_naive(levels: &Vec<i32>) -> bool {
    if is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut variant = levels.clone();
        variant.remove(i);

        if is_safe(&variant) {
            return true;
        }
    }

    false
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let (is_safe, expected_direction) = is_safe_transition(levels[0], levels[1], None);

    if !is_safe {
        return false;
    }

    for i in 1..levels.len() - 1 {
        let (safe, _) = is_safe_transition(levels[i], levels[i + 1], Some(expected_direction));
        if !safe {
            return false;
        }
    }

    true
}
