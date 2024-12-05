use std::cmp::Ordering;
use std::collections::HashMap;
use util::measure;

pub type RuleSet = HashMap<i32, Vec<i32>>;

#[derive(Debug, PartialEq, Eq)]
enum ComparisonResult {
    Equal,
    LessThan,
    GreaterThan,
    Unknown,
}

impl Into<Ordering> for ComparisonResult {
    fn into(self) -> Ordering {
        match self {
            Self::Equal => Ordering::Equal,
            Self::LessThan => Ordering::Less,
            Self::GreaterThan => Ordering::Greater,
            Self::Unknown => panic!("Unresolved ordering!"),
        }
    }
}

fn main() {
    println!("Advent of code 2024 - day 5!");
    let input = include_str!("../input.txt");

    let (ruleset, page_lists) = parse_input(input);

    // Validate all comparisons are accounted for
    for k1 in ruleset.keys() {
        for k2 in ruleset.keys() {
            assert_ne!(cmp_pages(*k1, *k2, &ruleset), ComparisonResult::Unknown)
        }
    }

    measure("Part 1", 10, || {
        page_lists
            .iter()
            .map(|page_list| {
                if part1_pages_ordered(page_list, &ruleset) {
                    page_list[page_list.len() / 2]
                } else {
                    0
                }
            })
            .sum::<i32>()
    });

    // NOTE: Measurement here will be a little off because of the clone we have to do because the function is not idempotent
    measure(
        "Part 2 (with cloning and sorting, not a good measurement)",
        10,
        || {
            page_lists
                .clone()
                .iter_mut()
                .map(|page_list| {
                    if !part1_pages_ordered(page_list, &ruleset) {
                        page_list.sort_by(|a, b| cmp_pages(*a, *b, &ruleset).into());
                        page_list[page_list.len() / 2]
                    } else {
                        0
                    }
                })
                .sum::<i32>()
        },
    );
}

fn part1_pages_ordered(page_list: &Vec<i32>, ruleset: &RuleSet) -> bool {
    for i in 1..page_list.len() {
        let (a, b) = (page_list[i - 1], page_list[i]);
        match cmp_pages(a, b, &ruleset) {
            ComparisonResult::Equal | ComparisonResult::GreaterThan => return false,
            ComparisonResult::Unknown => panic!("Undecided entry found: {a} <=> {b}"),
            ComparisonResult::LessThan => (),
        }
    }

    true
}

fn cmp_pages(a: i32, b: i32, ruleset: &RuleSet) -> ComparisonResult {
    // Try to find a path from a -> b
    if a == b {
        return ComparisonResult::Equal;
    }

    if let Some(a_less_than) = ruleset.get(&a) {
        if a_less_than.contains(&b) {
            return ComparisonResult::LessThan;
        }
    }

    if let Some(b_less_than) = ruleset.get(&b) {
        if b_less_than.contains(&a) {
            return ComparisonResult::GreaterThan;
        }
    }

    ComparisonResult::Unknown
}

fn parse_input(input: &str) -> (RuleSet, Vec<Vec<i32>>) {
    let mut parsing_rules = true;

    let mut ruleset = RuleSet::new();
    let mut page_list = vec![];

    for line in input.lines() {
        if line.is_empty() {
            parsing_rules = false;
        } else if parsing_rules {
            let mut rule_split = line.split('|');
            let (val, less_than): (i32, i32) = (
                rule_split.next().unwrap().parse().unwrap(),
                rule_split.next().unwrap().parse().unwrap(),
            );

            if let Some(val_less_than) = ruleset.get_mut(&val) {
                val_less_than.push(less_than);
            } else {
                ruleset.insert(val, vec![less_than]);
            }
        } else {
            page_list.push(
                line.split(',')
                    .map(|i| i.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            )
        }
    }

    (ruleset, page_list)
}
