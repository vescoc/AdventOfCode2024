fn main() {
    #[cfg(feature = "input")]
    println!("cargo::rerun-if-changed=../input");

    #[cfg(feature = "input")]
    aoc::get_input(2024, 16, "../input");
}
