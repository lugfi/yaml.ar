use reflection::Yaml;
use std::fs::File;
use std::io::BufReader;
use lambda_calculus::*;
use lambda_calculus::data::num::church;
use std::path::Path;

use std::error::Error;

pub fn read_term_from_file<P: AsRef<Path>>(path: P) -> Result<Yaml, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_yaml::from_reader(reader)?)
}

pub fn to_lambda_calculus(yaml_term: &Yaml) -> Term {
    match yaml_term {
        Yaml::String(var) => match var.as_ref() {
            "+" => church::add(),
            "-" => church::sub(),
            _ => panic!("variables are numbers because we use de Bruijn notation"),
        }
        Yaml::Integer(var) => var.into_church(),
        // https://www.cs.cornell.edu/courses/cs4110/2018fa/lectures/lecture15.pdf
        Yaml::Hash(_) => panic!("variables are numbers because we use de Bruijn notation"),
        Yaml::Array(vec) => match vec.as_slice() {
            [term] => to_lambda_calculus(term),
            [lhs, rhs] => app(to_lambda_calculus(lhs), to_lambda_calculus(rhs)),
            [t0, t1, t2] => app!(to_lambda_calculus(t0), to_lambda_calculus(t1), to_lambda_calculus(t2)),
            [t0, t1, t2, t3] => app!(to_lambda_calculus(t0), to_lambda_calculus(t1), to_lambda_calculus(t2), to_lambda_calculus(t3)),
            _ => panic!("unsupported")
        },
    }
}
