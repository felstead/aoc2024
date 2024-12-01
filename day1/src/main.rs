use std::{collections::BinaryHeap, fs, hint::black_box, io::Read, time};

fn main() {
    println!();
    println!("Advent of code 2024 - day 1!");
    println!("  - Min Heap Result : {}", streaming_min_heap());
    println!("  - Naive Result    : {}", streaming_min_heap());

    measure("MinHeap", 10, || {
        streaming_min_heap();
    });

    measure("Naive", 10, || {
        naive();
    });
}

const EXPECTED_LEN: usize = 1024;

fn streaming_min_heap() -> u32 {
    let input = get_input();

    let mut heap1 = BinaryHeap::with_capacity(EXPECTED_LEN);
    let mut heap2 = BinaryHeap::with_capacity(EXPECTED_LEN);
    input
        .split_ascii_whitespace()
        .enumerate()
        .for_each(|(index, str)| {
            let i = str.parse::<u32>().unwrap();
            #[rustfmt::skip]
            (if index % 2 == 0 { &mut heap1 } else { &mut heap2 }).push(i);
        });

    let mut sum = 0;
    while !heap1.is_empty() {
        let (left, right) = (heap1.pop().unwrap(), heap2.pop().unwrap());
        let diff = left.abs_diff(right);
        sum += diff;
    }

    sum
}

fn naive() -> u32 {
    let input = get_input();

    let mut list1 = Vec::with_capacity(EXPECTED_LEN);
    let mut list2 = Vec::with_capacity(EXPECTED_LEN);

    input
        .split_ascii_whitespace()
        .enumerate()
        .for_each(|(index, str)| {
            let i = str.parse::<u32>().unwrap();
            #[rustfmt::skip]
            (if index % 2 == 0 { &mut list1 } else { &mut list2 }).push(i);
        });

    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2.iter())
        .fold(0, |sum, (l, r)| sum + l.abs_diff(*r))
}

fn get_input() -> String {
    let mut input = String::new();
    fs::File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    input
}

pub fn measure<F: Fn()>(label: &str, iterations: usize, f: F) {
    let mut times = Vec::new();

    // Warm up
    black_box(f());

    for _ in 0..iterations {
        let start = time::Instant::now();
        black_box(f());
        let end = time::Instant::now();
        times.push(end - start);
    }

    times.sort();
    let (min, median, max) = (times[0], times[iterations / 2], times[iterations - 1]);

    println!(
        "{}: Median time: {:?}   (min: {:?} / max: {:?})",
        label, median, min, max
    );
}
