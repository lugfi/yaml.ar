use reflection::YamlTerm;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use std::error::Error;

// fn eval(yaml_term: &YamlTerm, env: &mut risp::RispEnv) -> Result<YamlTerm, risp::RispErr> {
//     match risp::eval(&to_risp(yaml_term), env) {
//         Ok(res) => Ok(YamlTerm::Var(res.to_string())),
//         Err(e) => Err(e),
//     }
// }

pub fn read_term_from_file<P: AsRef<Path>>(path: P) -> Result<YamlTerm, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_yaml::from_reader(reader)?)
}

pub fn to_risp(yaml_term: &YamlTerm) -> risp::RispExp {
    match yaml_term {
        YamlTerm::Var(var) => risp::RispExp::Symbol(var.to_string()),
        YamlTerm::VarF64(var) => risp::RispExp::Number(*var),
        YamlTerm::Abs(hm) => {
            let (key, val) = hm.iter().next().unwrap();
            risp::RispExp::List(vec![
                risp::RispExp::Symbol("fn".to_string()),
                risp::RispExp::List(vec![risp::RispExp::Symbol(key.to_string())]),
                to_risp(val),
            ])
        }
        YamlTerm::App(vec) => match vec.as_slice() {
            [] => panic!(),
            [term] => to_risp(term),
            terms => risp::RispExp::List(terms.iter().map(|term| to_risp(term)).collect()),
        },
    }
}
