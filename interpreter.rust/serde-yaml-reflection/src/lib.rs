use serde::Deserialize;

use std::collections::HashMap;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum YamlTerm {
    Var(String),
    VarF64(f64),
    Abs(HashMap<String, YamlTerm>),
    App(Vec<YamlTerm>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::Token;

    #[test]
    fn variable_term() {
        serde_test::assert_de_tokens(
            &YamlTerm::Var("variable".to_owned()),
            &[Token::Str("variable")],
        );
    }

    #[test]
    fn abstraction_of_a_single_variable() {
        serde_test::assert_de_tokens(
            &YamlTerm::Abs({
                let mut map = HashMap::new();
                map.insert(
                    "bound_variable".to_owned(),
                    YamlTerm::Var("target_expression".to_owned()),
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
            &YamlTerm::App(vec![
                YamlTerm::Var("x".to_owned()),
                YamlTerm::Var("y".to_owned()),
            ]),
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
