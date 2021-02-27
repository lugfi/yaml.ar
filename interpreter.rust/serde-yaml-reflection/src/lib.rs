use serde::{Deserialize, Serialize};

use std::collections::HashMap;

// https://docs.rs/yaml-rust/0.4.5/yaml_rust/yaml/enum.Yaml.html
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Yaml {
    Integer(usize),
    // Integer(i64),
    String(String),
    Array(Vec<Yaml>),
    Hash(HashMap<String, Yaml>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::Token;

    #[test]
    fn variable_term() {
        serde_test::assert_de_tokens(
            &Yaml::String("variable".to_owned()),
            &[Token::Str("variable")],
        );
    }

    #[test]
    fn abstraction_of_a_single_variable() {
        serde_test::assert_de_tokens(
            &Yaml::Abs({
                let mut map = HashMap::new();
                map.insert(
                    "bound_variable".to_owned(),
                    Yaml::Var("target_expression".to_owned()),
                );
                map
            }),
            &[
                Token::Map { len: Some(1) },
                Token::Str("bound_variable"),
                Token::Str("target_expression"),
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn application_of_two_variables() {
        serde_test::assert_de_tokens(
            &Yaml::App(vec![Yaml::Var("x".to_owned()), Yaml::Var("y".to_owned())]),
            // this represents [ x, y ]
            &[
                Token::Seq { len: Some(2) },
                Token::Str("x"),
                Token::Str("y"),
                Token::SeqEnd,
            ],
        );
    }
}
