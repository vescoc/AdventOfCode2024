fn main() {
    println!("cargo::rerun-if-changed=../input");
    
    aoc::get_input_info_from_cargo(Some("../input".to_string()));
}
