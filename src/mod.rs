use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs::OpenOptions;
use std::io::prelude::*;

mod types {

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ResponseValueType {
        r#type: String,
        description: Option<String>,
        items: Option<Box<ResponseValueType>>,
        properties: Option<HashMap<String, ResponseValueType>>,
    }

    pub fn get_response_type(value: &ResponseValueType) -> String {
        match value.r#type.as_str() {
            "object" => {
                if let Some(properties) = &value.properties {
                    let mut result = String::from("{\n");
                    for (k, v) in properties {
                        result.push_str(&format!("{}:{}\n", k, get_type(v)));
                    }
                    result.push_str("\n}");
                    return result;
                }
                return String::new();
            },
            "array" => {
                if let Some(items) = &value.items {
                    return format!("{}[]", get_type(items));
                }
                return String::new();
            },
            "string" => {
                return String::from("string");
            },
            "integer" => {
                return String::from("number");
            },
            "boolean" => {
                return String::from("boolean");
            }
            _ => String::new(),
        }
    }

}