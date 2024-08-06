use std::collections::HashMap;

use anyhow::Result;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "json.pest"]
struct JsonParser;

#[allow(unused)]
#[derive(Debug, PartialEq)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn main() -> Result<()> {
    let s = r#"{
        "name": "John Doe",
        "age": 30,
        "is_student": false,
        "marks": [90, -80, 85.1],
        "address": {
            "city": "New York",
            "zip": 10001
        }
    }"#;
    let parsed = JsonParser::parse(Rule::json, s)?.next().unwrap();
    let v = parse_value(parsed)?;
    println!("{:#?}", v);
    Ok(())
}

fn parse_value(pair: Pair<Rule>) -> Result<JsonValue> {
    let ret = match pair.as_rule() {
        Rule::null => JsonValue::Null,
        Rule::bool => JsonValue::Bool(pair.as_str() == "true"),
        Rule::number => JsonValue::Number(pair.as_str().parse()?),
        Rule::chars => JsonValue::String(pair.as_str().to_string()),
        Rule::array => {
            let mut values = vec![];
            for inner_pair in pair.into_inner() {
                values.push(parse_value(inner_pair)?);
            }
            JsonValue::Array(values)
        }
        Rule::object => {
            let mut map = HashMap::new();
            let mut key = "";
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::pair => {
                        for pair_inner in inner_pair.into_inner() {
                            match pair_inner.as_rule() {
                                Rule::chars => {
                                    key = pair_inner.as_str();
                                }
                                Rule::value => {
                                    map.insert(key.to_string(), parse_value(pair_inner)?);
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
            JsonValue::Object(map)
        }
        Rule::value => {
            let inner_pair = pair.into_inner().next().unwrap();
            parse_value(inner_pair)?
        }
        v => {
            panic!("unhandled rule: {:?}", v);
        }
    };
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use pest::consumes_to;
    use pest::parses_to;

    use super::*;

    #[test]
    fn pest_parse_null_should_work() -> Result<()> {
        let s = "null";
        let parsed = JsonParser::parse(Rule::null, s)?.next().unwrap();
        let v = parse_value(parsed)?;
        assert_eq!(v, JsonValue::Null);
        Ok(())
    }

    #[test]
    fn pest_parse_bool_should_work() -> Result<()> {
        let s = "true";
        let parsed = JsonParser::parse(Rule::bool, s)?.next().unwrap();
        let v = parse_value(parsed)?;
        assert_eq!(v, JsonValue::Bool(true));

        let s = "false";
        let parsed = JsonParser::parse(Rule::bool, s)?.next().unwrap();
        let v = parse_value(parsed)?;
        assert_eq!(v, JsonValue::Bool(false));
        Ok(())
    }

    #[test]
    fn pest_parse_number_should_work() -> Result<()> {
        let s = "123";
        let parsed = JsonParser::parse(Rule::number, s)?.next().unwrap();
        let v = parse_value(parsed)?;
        assert_eq!(v, JsonValue::Number(123.0));

        let s = "-123";
        let parsed = JsonParser::parse(Rule::number, s)?.next().unwrap();
        let v = parse_value(parsed)?;
        assert_eq!(v, JsonValue::Number(-123.0));

        let s = "123.45";
        let parsed = JsonParser::parse(Rule::number, s)?.next().unwrap();
        let v = parse_value(parsed)?;
        assert_eq!(v, JsonValue::Number(123.45));
        Ok(())
    }

    #[test]
    fn pest_parse_string_should_work() -> Result<()> {
        let s = r#""hello""#;
        let parsed = JsonParser::parse(Rule::string, s)?.next().unwrap();
        let v = parse_value(parsed)?;
        assert_eq!(v, JsonValue::String("hello".to_string()));
        Ok(())
    }

    #[test]
    fn pest_parse_array_should_work() -> Result<()> {
        let s = r#"[1, 2, 3]"#;
        let parsed = JsonParser::parse(Rule::array, s)?.next().unwrap();
        let v = parse_value(parsed)?;
        assert_eq!(
            v,
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0)
            ])
        );
        Ok(())
    }

    #[test]
    fn pest_parse_object_should_work() -> Result<()> {
        let s = r#"{"name": "John Doe", "age": 30}"#;
        let parsed = JsonParser::parse(Rule::object, s)?.next().unwrap();
        let v = parse_value(parsed)?;
        let mut map = HashMap::new();
        map.insert(
            "name".to_string(),
            JsonValue::String("John Doe".to_string()),
        );
        map.insert("age".to_string(), JsonValue::Number(30.0));
        assert_eq!(v, JsonValue::Object(map));
        Ok(())
    }

    #[test]
    fn pest_parse_ruls_should_work() {
        parses_to! {
            parser: JsonParser,
            input: r#"{"hello" : "world"}"#,
            rule: Rule::json,
            tokens: [
                    object(0, 19, [
                        pair(1, 18, [
                            chars(2, 7),
                            value(11, 18, [
                                chars(12, 17)
                            ])
                        ])
                    ])
            ]
        };
    }
}
