use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize)]
#[serde(untagged)]
enum YamlTerm {
    Var(String),
    Abs(HashMap<String, YamlTerm>), // only one key
    App(Vec<YamlTerm>),             // only two values
}

impl YamlTerm {
    fn to_term(&self) -> Term {
        match self {
            YamlTerm::Var(var) => Term::Var(var.as_bytes()[0]),
            YamlTerm::Abs(hm) => {
                let (key, val) = hm.iter().next().unwrap();
                Term::Abs(key.as_bytes()[0], Box::new(val.to_term()))
            }
            YamlTerm::App(vec) => match vec.as_slice() {
                [] => panic!(),
                [term] => term.to_term(),
                [term0, vec @ ..] => vec.iter().fold(term0.to_term(), |acc, t| {
                    Term::App(Box::new(acc), Box::new(t.to_term()))
                }),
            },
        }
    }
}

fn read_term_from_file<P: AsRef<Path>>(path: P) -> Result<YamlTerm, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_yaml::from_reader(reader)?)
}

fn main() {
    let file = read_term_from_file(Path::new("example.yaml")).unwrap();
    let term = file.to_term();

    println!("Original term: {}", term);
    println!("After reduction: {}", term.reduce());
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
                if var == *arg {
                    Term::Abs(*arg, Box::new(body.replace(var, subs)))
                } else {
                    body.replace(var, subs)
                }
            }
            Term::App(t1, t2) => Term::App(
                Box::new(t1.replace(var, subs.clone())),
                Box::new(t2.replace(var, subs)),
            ),
        }
    }

    /// Reduce `self` if possible.
    fn reduce(self) -> Term {
        match self {
            // beta-reduction
            Term::App(t1, t2) => match t1.as_ref() {
                Term::Abs(var, body) => body.replace(*var, t2),
                _ => Term::App(Box::new(t1.reduce()), Box::new(t2.reduce())),
            },
            Term::Abs(_, term) => term.reduce(),
            t => t,
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
