use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let fileArg = env::args().nth(1);
    match fileArg {
        Some(file) => println!("Juuh! {:?}", file),
        None => {
            println!("Usage: cargo run file.syx");
            std::process::exit(-1);
        }
    }
}
