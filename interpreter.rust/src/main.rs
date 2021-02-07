use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use std::collections::HashMap;
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct YamlYaml {
    example: YamlTerm,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum YamlTerm {
    Var(String),
    Abs(HashMap<String, YamlTerm>),    // only one key
    App(Box<YamlTerm>, Box<YamlTerm>), // only two values
}

impl YamlTerm {
    fn to_term(&self) -> Term {
        match self {
            YamlTerm::Var(var) => Term::Var(var.as_bytes()[0]),
            YamlTerm::Abs(hm) => {
                if let Some((key, val)) = hm.iter().next() {
                    Term::Abs(key.as_bytes()[0], Box::new(val.to_term()))
                } else {
                    panic!()
                }
            }
            // Term::Abs(hm.entry(0).key().as_bytes()[0], hm.entry(0).)),
            YamlTerm::App(t0, t1) => Term::App(Box::new(t0.to_term()), Box::new(t1.to_term())),
        }
    }
}

fn read_term_from_file<P: AsRef<Path>>(path: P) -> Result<YamlYaml, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_yaml::from_reader(reader)?)
}

fn main() {
    let file = read_term_from_file(Path::new("example.yaml")).unwrap();
    let mut term = file.example.to_term();
    // TODO map yaml_yaml to term
    // let mut term = Term::App(
    //     Box::new(Term::Abs(
    //         b'x',
    //         Box::new(Term::App(
    //             Box::new(Term::Var(b'y')),
    //             Box::new(Term::Var(b'x')),
    //         )),
    //     )),
    //     Box::new(Term::Var(b'z')),
    // );

    println!("Original term: {}", term);
    term.reduce();
    println!("After reduction: {}", term);
}

// https://christianpoveda.github.io/blog/untyped-lambda-calculus/

use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
enum Term {
    Var(u8),
    Abs(u8, Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl Term {
    /// Decide if `var` is free in `self`.
    fn is_free(&self, var: u8) -> bool {
        match self {
            Term::Var(var2) => var == *var2,
            Term::Abs(arg, body) => (var != *arg) && body.is_free(var),
            Term::App(t1, t2) => t1.is_free(var) || t2.is_free(var),
        }
    }
    /// Replace `var` by `subs` inside `self`. Return `false` if the substitution
    /// did not take place due to issues with free variables.
    fn replace(&mut self, var: u8, subs: &Term) -> bool {
        match self {
            Term::Var(var2) => {
                if var == *var2 {
                    *self = subs.clone();
                }
                true
            }
            Term::Abs(arg, body) => {
                if var == *arg {
                    true
                } else if subs.is_free(*arg) {
                    false
                } else {
                    body.replace(var, subs)
                }
            }
            Term::App(t1, t2) => t1.replace(var, subs) && t2.replace(var, subs),
        }
    }

    /// Reduce `self` if possible.
    fn reduce(&mut self) {
        match self {
            // beta-reduction
            Term::App(t1, t2) => match &mut **t1 {
                Term::Abs(var, body) => {
                    if body.replace(*var, t2) {
                        *self = *body.clone();
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(var) => write!(f, "{}", *var as char),
            Term::Abs(var, term) => write!(f, "(λ{}. {})", *var as char, term),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
        }
    }
}
