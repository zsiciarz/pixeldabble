fn run() -> Result<(), String> {
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);
        std::process::exit(1);
    }
}
