extern crate pixeldabble;

fn main() {
    let image_file = std::env::args().nth(1).unwrap_or_else(|| {
        println!("USAGE: pixeldabble <FILENAME>");
        std::process::exit(1);
    });
    if let Err(ref e) = pixeldabble::run(&image_file[..]) {
        println!("error: {}", e);
        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}",
                     backtrace);
        }
        ::std::process::exit(1);
    }
}
