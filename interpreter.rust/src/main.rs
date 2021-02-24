use interpreter::eval_file;
use std::path::Path;

fn main() {
    eval_file(Path::new("examples/e1.yaml"));
    eval_file(Path::new("examples/e2.yaml"));
    eval_file(Path::new("examples/e3.yaml"));
    eval_file(Path::new("examples/e4-abstraction.yaml"));
}
