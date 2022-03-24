use std::env;
use lib::fowmat;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", fowmat(args.join(" ").as_str()))
}