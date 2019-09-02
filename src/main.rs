fn main() {
    let args: Vec<String> = std::env::args().collect();
    let query = &args[1];
    let file_name = &args[2];
    println!("query: {}", query);
    println!("file name: {}", file_name);
}
