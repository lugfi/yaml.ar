use reflection::YamlTerm;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use std::error::Error;

fn to_term(yaml_term: &YamlTerm) -> Term {
    match yaml_term {
        YamlTerm::Var(var) => Term::Var(var.as_bytes()[0]),
        YamlTerm::Abs(hm) => {
            let (key, val) = hm.iter().next().unwrap();
            Term::Abs(key.as_bytes()[0], Box::new(to_term(val)))
        }
        YamlTerm::App(vec) => match vec.as_slice() {
            [] => panic!(),
            [term] => to_term(term),
            [term0, vec @ ..] => vec.iter().fold(to_term(term0), |acc, t| {
                Term::App(Box::new(acc), Box::new(to_term(t)))
            }),
        },
    }
}
fn to_risp(yaml_term: &YamlTerm) -> risp::RispExp {
    match yaml_term {
        YamlTerm::Var(var) => risp::RispExp::Symbol(var.to_string()),
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

fn read_term_from_file<P: AsRef<Path>>(path: P) -> Result<YamlTerm, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_yaml::from_reader(reader)?)
}

fn main() {
    let file = read_term_from_file(Path::new("example.yaml")).unwrap();
    let term = to_term(&file);
    println!("Original term: {}", term);
    println!("After reduction: {}", term.reduce());

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

// https://christianpoveda.github.io/blog/untyped-lambda-calculus/

use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
enum Term {
    Unit,
    Var(u8),
    Abs(u8, Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl Term {
    /// Decide if `var` is free in `self`.
    fn is_free(&self, var: u8) -> bool {
        match self {
            Term::Unit => false,
            Term::Var(var2) => var == *var2,
            Term::Abs(arg, body) => (var != *arg) && body.is_free(var),
            Term::App(t1, t2) => t1.is_free(var) || t2.is_free(var),
        }
    }
    /// Replace `var` by `subs` inside `self`
    fn replace(&self, var: u8, subs: Box<Term>) -> Term {
        match self {
            Term::Unit => Term::Unit,
            Term::Var(var2) => {
                if var == *var2 {
                    subs.as_ref().clone()
                } else {
                    Term::Var(*var2)
                }
            }
            Term::Abs(arg, body) => {
                if var != *arg && !subs.is_free(*arg) {
                    body.replace(var, subs)
                } else {
                    Term::Abs(*arg, Box::new(body.as_ref().clone()))
                }
            }
            Term::App(t1, t2) => Term::App(
                Box::new(t1.replace(var, subs.clone())),
                Box::new(t2.replace(var, subs.clone())),
            ),
        }
    }

    // https://en.wikipedia.org/wiki/Lambda_calculus#%CE%B2-reduction
    fn reduce(self) -> Term {
        match self {
            Term::App(t1, t2) => match t1.as_ref() {
                Term::Abs(var, body) => body.replace(*var, t2),
                _ => Term::App(t1, t2),
            },
            _ => self,
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Unit => write!(f, "()"),
            Term::Var(var) => write!(f, "{}", *var as char),
            Term::Abs(var, term) => write!(f, "(λ{}. {})", *var as char, term),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
        }
    }
}
