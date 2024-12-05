fn main() {
    println!("cargo::rerun-if-changed=../input");
    
    aoc::get_input(2025, 5, "../input");
}
