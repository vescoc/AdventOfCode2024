fn main() {
    println!("cargo::rerun-if-changed=../input");

    aoc::get_input(2024, 4, "../input");
}
