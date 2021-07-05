use interpreter::eval_lambda_calculus;
use std::env;
use std::path::Path;

fn main() {
    match env::args().skip(1).next() {
        Some(path) => eval_lambda_calculus(Path::new(&path)),
        None => println!("Usage:\n  cargo run --bin lambda -- [path_to_yaml]"),
    }
}
