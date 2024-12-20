use std::time::Instant;

#[cfg(feature = "input")]
fn main() {
    let now = Instant::now();

    println!("part 1: {}", day20::part_1());
    println!("part 2: {}", day20::part_2());

    let elapsed = now.elapsed();
    println!(
        "elapsed: {}ms ({}us, {}ns)",
        elapsed.as_millis(),
        elapsed.as_micros(),
        elapsed.as_nanos()
    );
}

#[cfg(not(feature = "input"))]
fn main() {
    let now = Instant::now();

    println!("part 1: {}", day20::solve_1(&INPUT));
    println!("part 2: {}", day20::solve_2(&INPUT));

    let elapsed = now.elapsed();
    println!(
        "elapsed: {}ms ({}us, {}ns)",
        elapsed.as_millis(),
        elapsed.as_micros(),
        elapsed.as_nanos()
    );
}
