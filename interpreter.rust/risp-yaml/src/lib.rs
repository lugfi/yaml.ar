use reflection::Yaml;
use risp::types::RispType;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use std::error::Error;

pub fn read_term_from_file<P: AsRef<Path>>(path: P) -> Result<Yaml, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_yaml::from_reader(reader)?)
}

pub fn to_risp(yaml_term: &Yaml) -> RispType {
    match yaml_term {
        Yaml::String(var) => RispType::Symbol(var.to_string()),
        Yaml::Integer(var) => RispType::Int(*var),
        Yaml::Hash(hm) => {
            let (key, val) = hm.iter().next().unwrap();
            RispType::List(vec![
                RispType::Symbol("defn".to_string()),
                RispType::Symbol(key.to_string()),
                RispType::Vector(vec![RispType::Symbol(key.to_string())]),
                to_risp(val),
            ])
        }
        Yaml::Array(vec) => match vec.as_slice() {
            [term] => to_risp(term),
            terms => RispType::List(terms.iter().map(|term| to_risp(term)).collect()),
        },
    }
}
