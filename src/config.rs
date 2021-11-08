use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Config {
    pub command: Vec<String>,
    pub questions: Vec<Question>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Question {
    pub key: String,
    pub question_text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_deserialize_a_config() {
        let config = r#"
command = ["target/debug/command", "output", "-f", "json"]
questions = [
  { key = "question_0", question_text = "Why?" }
]
"#;
        let expected = Config {
            command: vec![
                "target/debug/command".to_owned(),
                "output".to_owned(),
                "-f".to_owned(),
                "json".to_owned(),
            ],
            questions: vec![Question {
                key: "question_0".to_owned(),
                question_text: "Why?".to_owned(),
            }],
        };

        let actual: Config = toml::from_str(config).expect("Failed to parse toml string");

        assert_eq!(actual, expected);
    }
}
