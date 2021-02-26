use interpreter::eval_file;
use risp::core::create_core_environment;
use std::env;
use std::path::Path;

fn main() {
    let env = &mut create_core_environment();
    match env::args().skip(1).next() {
        Some(path) => eval_file(Path::new(&path), env),
        None => println!("Usage:\n  cargo run --example eval-file -- [path_to_yaml]"),
    }
}
