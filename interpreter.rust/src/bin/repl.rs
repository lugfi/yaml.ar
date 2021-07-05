use risp::core::create_core_environment;
use risp::eval::eval;
use risp_yaml::to_risp;
use std::io;

fn main() {
    let env = &mut create_core_environment();
    loop {
        println!("risp-yaml >");
        let mut expr = String::new();
        while io::stdin().read_line(&mut expr).expect("valid code") > 1 {}

        let yaml_expr = serde_yaml::from_str(&expr).expect("invalid yaml line");
        let risp_expr = to_risp(&yaml_expr);
        match eval(risp_expr, env) {
            Ok(res) => println!("🔥 {:?}", res),
            Err(e) => println!("🙀 {:?}", e),
        }
    }
}
