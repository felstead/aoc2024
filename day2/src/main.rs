use std::fs::read_to_string;
use util::measure;

fn main() {
    println!("Advent of code 2024 - day 2!");

    let levels = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(split_line_to_levels)
        .collect::<Vec<_>>();

    measure("Num safe (part1)", 10, || {
        levels
        .iter()
        .fold(0, |acc, l| acc + if is_safe(l) { 1 } else { 0 })
    });

    measure("Num safe (part 2 naive)", 10, || {
        levels.iter().fold(0, |acc, l| {
            acc + if is_safe_with_dampener_naive(l) { 1 } else { 0 }
        })
    });

    let masks_by_level = generate_masks(10);
    measure("Num safe (part 2 with bitmasks)", 10, || {
        levels.iter().fold(0, |acc, l| {
            acc + if is_safe_with_dampener_bitmasks(l, &masks_by_level[l.len() - 1]) {
                1
            } else {
                0
            }
        })
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Asc,
    Desc,
    None,
}

fn get_direction_and_diff(a: i32, b: i32) -> (Direction, i32) {
    let diff = a.abs_diff(b);
    #[allow(clippy::comparison_chain)]
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

fn is_safe_with_dampener_naive(levels: &[i32]) -> bool {
    if is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut variant: Vec<i32> = levels.into();
        variant.remove(i);

        if is_safe(&variant) {
            return true;
        }
    }

    false
}

fn is_safe(levels: &[i32]) -> bool {
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

fn generate_masks(num_delta_levels: usize) -> Vec<Vec<u32>> {
    let mut masks_by_level = vec![];

    for level in 0..num_delta_levels {
        if level < 3 {
            masks_by_level.push(vec![])
        } else {
            let single_level_mask = 2u32.pow((level) as u32) - 1;
            let mut masks = vec![
                // Single level undampened, i.e. all values from 0..num_levels-1
                single_level_mask,
                // Dampened with first element removed
                single_level_mask - 1,
                single_level_mask - (1 << (level - 1)),
            ];

            for i in 1..level {
                // Remove ith and i-1th element, replace with i+num_delta_levels element
                masks.push(single_level_mask - (1 << i) - (1 << (i - 1)) + (1 << (i + level - 1)));
            }

            masks_by_level.push(masks);
        }
    }

    masks_by_level
}

fn is_safe_with_dampener_bitmasks(levels: &[i32], masks: &[u32]) -> bool {
    assert_eq!(masks.len(), levels.len() + 1);

    // Calculate deltas for i - i+1 and also i - i+2
    let num_delta_levels = levels.len() - 1;
    let mut valid = 0u32;
    let mut sign = 0u32;

    for i in 0..num_delta_levels {
        // Single level delta
        let delta = levels[i] - levels[i + 1];
        valid |= ((delta.abs() > 0 && delta.abs() <= 3) as u32) << i;
        sign |= ((delta > 0) as u32) << i;

        // Double level delta
        if i < num_delta_levels - 1 {
            let delta = levels[i] - levels[i + 2];
            //deltas2.push(delta);
            valid |= ((delta.abs() > 0 && delta.abs() <= 3) as u32) << (i + num_delta_levels);
            sign |= ((delta > 0) as u32) << (i + num_delta_levels);
        }
    }

    for mask in masks {
        let masked_valid = valid & *mask;
        let masked_sign = sign & *mask;

        let safe = (masked_valid == *mask) && (masked_sign == 0 || masked_sign == *mask);

        if safe {
            return true;
        }
    }

    false
}
