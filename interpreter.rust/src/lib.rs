use risp_yaml::read_term_from_file;
use risp_yaml::to_risp;
use std::path::Path;

pub fn eval_file(path: &Path) {
    let file = read_term_from_file(path).unwrap();

    let file_risp = to_risp(&file);
    let env = &mut risp::default_env();
    match risp::eval(&file_risp, env) {
        Ok(res) => println!("// 🔥 => {}", res),
        Err(e) => match e {
            risp::RispErr::Reason(msg) => {
                println!("// 🙀 => {}", msg);
                println!("{}", file_risp);
            }
        },
    }
}
