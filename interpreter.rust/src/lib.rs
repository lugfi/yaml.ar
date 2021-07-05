use lambda_calculus::reduction::*;
use lambda_calculus_yaml::to_lambda_calculus;
use risp::environment::Environment;
use risp::eval::eval;
use risp_yaml::read_term_from_file;
use risp_yaml::to_risp;
use std::path::Path;

pub fn eval_file(path: &Path, env: &mut Environment) {
    let file = read_term_from_file(path).unwrap();

    let file_risp = to_risp(&file);
    match eval(file_risp, env) {
        Ok(res) => println!("// 🔥 => {:?}", res),
        Err(e) => println!("// 🙀 => {:?}", e),
    }
}

pub fn eval_lambda_calculus(path: &Path) {
    let file = read_term_from_file(path).unwrap();

    let mut expr = to_lambda_calculus(&file);
    println!("steps: {}", expr.reduce(NOR, 0));
    println!("{}", expr);
}
