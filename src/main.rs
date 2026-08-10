use std::env;

fn main() {
    if env::args().len() != 2 {
        eprintln!("Incorrect number of arguments. Please provide only one argument.");
        std::process::exit(1);
    }
    let arg = env::args().nth(1).unwrap();
    println!("{arg}");
}
