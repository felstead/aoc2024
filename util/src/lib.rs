use std::{hint::black_box, time};

pub fn measure<F: Fn()>(label: &str, iterations: usize, f: F) {
    let mut times = Vec::new();

    // Warm up
    #[allow(clippy::unit_arg)]
    black_box(f());

    for _ in 0..iterations {
        let start = time::Instant::now();
        #[allow(clippy::unit_arg)]
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
