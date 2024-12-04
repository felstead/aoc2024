use std::sync::LazyLock;
use regex::Regex;

fn main() {
    println!("Advent of code 2024 - day 3!");

    let input = std::fs::read_to_string("input.txt").unwrap();


    util::measure("Part 1 (regex)", 10, || {
        part1(&input)
    });

    util::measure("Part 2 (regex with do/don't)", 10, || {
        part2(&input)
    });
}

fn part1(input : &str) -> i32 {
    static MATCHER : LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());
    
    MATCHER.captures_iter(&input).map(|cap| {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        a * b
    }).fold(0, |acc, x| acc + x)
}

fn part2(input : &str) -> i32 {
    static MATCHER : LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?<do>do\(\))|(?<mul>mul\((?<a>\d{1,3}),(?<b>\d{1,3})\))|(?<dont>don't\(\))").unwrap());

    let mut do_capture = true;
    MATCHER.captures_iter(&input).map(|cap| {
        //println!("{:?}", cap.get(0));
        if let Some(_) = cap.name("mul") {
            if do_capture {
                let a = cap.name("a").unwrap().as_str().parse::<i32>().unwrap();
                let b = cap.name("b").unwrap().as_str().parse::<i32>().unwrap();
                //println!("RESULT: {}", a * b * capture_multiplier);
                return a * b
            } else {
                return 0
            }
        } else if cap.name("do").is_some() {
            //println!("ENABLE");
            do_capture = true
        } else if cap.name("dont").is_some() {
            //println!("DISABLE");
            do_capture = false
        }
        0
    }).fold(0, |acc, x| acc + x)
}