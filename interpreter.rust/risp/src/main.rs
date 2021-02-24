use std::io;

fn parse_eval(expr: String, env: &mut risp::RispEnv) -> Result<risp::RispExp, risp::RispErr> {
    let (parsed_exp, _) = risp::parse(&risp::tokenize(expr))?;
    let evaled_exp = risp::eval(&parsed_exp, env)?;

    Ok(evaled_exp)
}

fn slurp_expr() -> String {
    let mut expr = String::new();

    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");

    expr
}

fn main() {
    let env = &mut risp::default_env();
    loop {
        println!("risp >");
        let expr = slurp_expr();
        match parse_eval(expr, env) {
            Ok(res) => println!("// 🔥 => {}", res),
            Err(e) => match e {
                risp::RispErr::Reason(msg) => println!("// 🙀 => {}", msg),
            },
        }
    }
}
