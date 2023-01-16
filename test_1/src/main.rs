use catibo;



fn main() {
    let input_image = std::fs::read("./data/_1mmCube.ctb").unwrap();
    let parsed = catibo::input::parse_file(&input_image).unwrap();
    println!("Read Cube is {:?}", parsed.machine_type);
}
