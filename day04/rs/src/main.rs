use std::time::Instant;

#[cfg(feature = "input")]
fn main() {
    let now = Instant::now();

    println!("part 1: {}", day04::part_1());
    println!("part 2: {}", day04::part_2());

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
    use std::io;

    let input = io::read_to_string(io::stdin()).expect("cannot read input");
    
    let now = Instant::now();

    println!("part 1: {}", day04::solve_1());
    println!("part 2: {}", day04::solve_2());

    let elapsed = now.elapsed();
    println!(
        "elapsed: {}ms ({}us, {}ns)",
        elapsed.as_millis(),
        elapsed.as_micros(),
        elapsed.as_nanos()
    );
}
