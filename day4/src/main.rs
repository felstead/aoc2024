use util::measure;
pub type ByteArray2D = Vec<Vec<u8>>;

fn main() {
    println!("Advent of code 2024 - day 4!");

    let input = include_str!("../input.txt");
    let haystack = parse_input(input);

    measure("Part 1 (naive array search)", 10, || {
        part1_naive_array_search(&haystack)
    });

    measure("Part 1 (naive array search reduced)", 10, || {
        part1_naive_array_search_reduced(&haystack)
    });

    measure("Part 1 (naive array extract_string)", 10, || {
        part1_naive_extract_string(&haystack)
    });

    measure("Part 1 (naive array extract_string reduced)", 10, || {
        part1_naive_extract_string_reduced(&haystack)
    });

    measure("Part 2 (naive search)", 10, || part2(&haystack));
}

fn parse_input(input: &str) -> ByteArray2D {
    let mut len: Option<usize> = None;
    input
        .lines()
        .map(|l| {
            let byte_vec: Vec<u8> = l.trim_ascii_end().as_bytes().into();
            // Validate that they all have the same length
            if len.is_none() {
                len = Some(byte_vec.len());
            } else {
                assert_eq!(len.unwrap(), byte_vec.len());
            }
            byte_vec
        })
        .collect::<Vec<_>>()
}

fn part1_naive_array_search(haystack: &ByteArray2D) -> usize {
    let mut count = 0;

    let width = haystack[0].len();
    let height = haystack.len();

    let needle = b"XMAS";

    // Iterate through each line and search in all eight directions
    for i in 0..width * height {
        let x = i % width;
        let y = i / width;

        if haystack[x][y] == needle[0] {
            count += Direction::iter_all()
                .filter(|dir| match_bytes_direction(haystack, needle, x, y, *dir))
                .count();
        }
    }

    count
}

fn part1_naive_array_search_reduced(haystack: &ByteArray2D) -> usize {
    let mut count = 0;

    let width = haystack[0].len();
    let height = haystack.len();

    let needle = b"XMAS";
    let needle_reversed = b"SAMX";

    // Iterate through each line and search in all eight directions
    for i in 0..width * height {
        let x = i % width;
        let y = i / width;

        let start = haystack[x][y];
        if start == needle[0] {
            count += Direction::iter_reduced()
                .filter(|dir| match_bytes_direction(haystack, needle, x, y, *dir))
                .count();
        }

        if start == needle_reversed[0] {
            count += Direction::iter_reduced()
                .filter(|dir| match_bytes_direction(haystack, needle_reversed, x, y, *dir))
                .count();
        }
    }

    count
}

fn part1_naive_extract_string(haystack: &ByteArray2D) -> usize {
    let mut count = 0;

    let width = haystack[0].len();
    let height = haystack.len();

    const NEEDLE: &[u8; 4] = b"XMAS";

    // Iterate through each line and search in all eight directions
    for i in 0..width * height {
        let x = i % width;
        let y = i / width;

        if haystack[x][y] == NEEDLE[0] {
            count += Direction::iter_all()
                .filter(|dir| {
                    if dir.can_needle_fit(NEEDLE.len(), x, y, width, height) {
                        let extracted = extract_string::<{ NEEDLE.len() }>(haystack, x, y, *dir);
                        &extracted == NEEDLE
                    } else {
                        false
                    }
                })
                .count();
        }
    }

    count
}

fn part1_naive_extract_string_reduced(haystack: &ByteArray2D) -> usize {
    let mut count = 0;

    let width = haystack[0].len();
    let height = haystack.len();

    const NEEDLE: &[u8; 4] = b"XMAS";
    const NEEDLE_REVERSED : &[u8;4] = b"SAMX";

    // Iterate through each line and search in all eight directions
    for i in 0..width * height {
        let x = i % width;
        let y = i / width;

        if haystack[x][y] == NEEDLE[0] || haystack[x][y] == NEEDLE_REVERSED[0] {
            count += Direction::iter_reduced()
                .filter(|dir| {
                    if dir.can_needle_fit(NEEDLE.len(), x, y, width, height) {
                        let extracted = extract_string::<{ NEEDLE.len() }>(haystack, x, y, *dir);
                        &extracted == NEEDLE || &extracted == NEEDLE_REVERSED
                    } else {
                        false
                    }
                })
                .count();
        }
    }

    count
}

fn part2(haystack: &ByteArray2D) -> usize {
    let mut count = 0;

    // We can skip the first and last columns
    let width = haystack[0].len();
    let height = haystack.len();

    for x in 1..width - 1 {
        for y in 1..height - 1 {
            if haystack[x][y] == b'A' {
                let tl = haystack[x - 1][y - 1];
                let tr = haystack[x + 1][y - 1];
                let bl = haystack[x - 1][y + 1];
                let br = haystack[x + 1][y + 1];

                let tl_br_match = (tl == b'S' && br == b'M') || (tl == b'M' && br == b'S');
                let tr_bl_match = (tr == b'S' && bl == b'M') || (tr == b'M' && bl == b'S');

                if tl_br_match && tr_bl_match {
                    count += 1;
                }
            }
        }
    }
    count
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    Up,
    UpRight,
}

impl Direction {
    pub fn can_needle_fit(
        &self,
        needle_len: usize,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> bool {
        let fits_right = x <= (width - needle_len);
        let fits_down = y <= (height - needle_len);
        let fits_left = x >= needle_len - 1;
        let fits_up = y >= needle_len - 1;
        match self {
            Direction::Right => fits_right,
            Direction::DownRight => fits_right && fits_down,
            Direction::Down => fits_down,
            Direction::DownLeft => fits_down && fits_left,
            Direction::Left => fits_left,
            Direction::UpLeft => fits_up && fits_left,
            Direction::Up => fits_up,
            Direction::UpRight => fits_up && fits_right,
        }
    }

    pub fn iter_all() -> impl Iterator<Item = Direction> {
        [
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft,
            Direction::Up,
            Direction::UpRight,
        ]
        .iter()
        .copied()
    }

    pub fn iter_reduced() -> impl Iterator<Item = Direction> {
        [
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
        ]
        .iter()
        .copied()
    }
}

fn extract_string<const N: usize>(
    haystack: &ByteArray2D,
    x: usize,
    y: usize,
    direction: Direction,
) -> [u8; N] {
    let mut result = [0u8; N];

    match direction {
        Direction::Right => (0..N).for_each(|i| result[i] = haystack[x + i][y]),
        Direction::DownRight => (0..N).for_each(|i| result[i] = haystack[x + i][y + i]),
        Direction::Down => (0..N).for_each(|i| result[i] = haystack[x][y + i]),
        Direction::DownLeft => (0..N).for_each(|i| result[i] = haystack[x - i][y + i]),
        Direction::Left => (0..N).for_each(|i| result[i] = haystack[x - i][y]),
        Direction::UpLeft => (0..N).for_each(|i| result[i] = haystack[x - i][y - i]),
        Direction::Up => (0..N).for_each(|i| result[i] = haystack[x][y - i]),
        Direction::UpRight => (0..N).for_each(|i| result[i] = haystack[x + i][y - i]),
    };

    result
}

fn match_bytes_direction(
    haystack: &ByteArray2D,
    needle: &[u8],
    x: usize,
    y: usize,
    direction: Direction,
) -> bool {
    let width: usize = haystack[0].len();
    let height = haystack.len();

    let mut it = needle.iter().enumerate();

    if !direction.can_needle_fit(needle.len(), x, y, width, height) {
        false
    } else {
        match direction {
            Direction::Right => it.all(|(i, b)| haystack[x + i][y] == *b),
            Direction::DownRight => it.all(|(i, b)| haystack[x + i][y + i] == *b),
            Direction::Down => it.all(|(i, b)| haystack[x][y + i] == *b),
            Direction::DownLeft => it.all(|(i, b)| haystack[x - i][y + i] == *b),
            Direction::Left => it.all(|(i, b)| haystack[x - i][y] == *b),
            Direction::UpLeft => it.all(|(i, b)| haystack[x - i][y - i] == *b),
            Direction::Up => it.all(|(i, b)| haystack[x][y - i] == *b),
            Direction::UpRight => it.all(|(i, b)| haystack[x + i][y - i] == *b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matcher() {
        let input = "S..S..S\n\
                           .A.A.A.\n\
                           ..MMM..\n\
                           SAMXMAS\n\
                           ..MMM..\n\
                           .A.A.A.\n\
                           S..S..S\n";

        let haystack = parse_input(input);
        let width = haystack[0].len();
        let height = haystack.len();

        let needle = b"XMAS";
        let needle_reversed = b"SAMX";
        Direction::iter_all().for_each(|dir| {
            assert!(match_bytes_direction(&haystack, needle, 3, 3, dir));
        });

        let mut count_all = 0;
        let mut count_reduced = 0;
        for i in 0..width * height {
            let x = i % width;
            let y = i / width;

            count_all += Direction::iter_all()
                .filter(|dir| match_bytes_direction(&haystack, needle, x, y, *dir))
                .count();

            count_reduced += Direction::iter_reduced()
                .filter(|dir| match_bytes_direction(&haystack, needle, x, y, *dir))
                .count();
            count_reduced += Direction::iter_reduced()
                .filter(|dir| match_bytes_direction(&haystack, needle_reversed, x, y, *dir))
                .count();
        }

        assert_eq!(count_all, count_reduced);
    }
}
