use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs::OpenOptions;
use std::io::prelude::*;
use crate::YapiObj;
use anyhow::{Result as AnyResult, Error};
use regex::Regex;

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseValueType {
    r#type: String,
    description: Option<String>,
    items: Option<Box<ResponseValueType>>,
    properties: Option<HashMap<String, ResponseValueType>>,
    #[serde(rename(deserialize = "$$ref"))]
    __ref: Option<String>,
}

fn get_response_type(value: &ResponseValueType) -> String {
    match value.r#type.as_str() {
        "object" => {
            if let Some(properties) = &value.properties {
                let mut result = String::from("{\n");
                for (k, v) in properties {
                    result.push_str(&format!("{}:{}\n", k, get_response_type(v)));
                }
                result.push_str("}");
                return result;
            }
            return String::from("{}");
        }
        "array" => {
            if let Some(items) = &value.items {
                return format!("{}[]", get_response_type(items));
            }
            return String::new();
        }
        "string" => {
            return String::from("string");
        }
        "integer" => {
            return String::from("number");
        }
        "boolean" => {
            return String::from("boolean");
        }
        _ => String::from("unknown"),
    }
}

pub fn generate(data: &Vec<YapiObj>) -> AnyResult<()> {
    let re = Regex::new(r"^\w+$").unwrap();
    let mut set = HashSet::new();
    let mut result = String::new();
    // TODO: refactor condition and loop
    for obj in data {
        for item in &obj.list {
            if item.res_body_type == "json" && re.is_match(&item.title) {
                if let Some(value) = &item.res_body {
                    if let Ok(v) = serde_json::from_str::<ResponseValueType>(value) {
                        let title = format!("{}{}", obj.name, item.title);
                        if !set.contains(&title) {
                            result = format!("{} \n export type {}{} = {}", result, obj.name, item.title, get_response_type(&v));
                            set.insert(title);
                        }
                    }
                }
            }
        }
    }
    if result.len() > 0 {
        match OpenOptions::new().create(true).read(true).write(true).open("type.ts") {
            Ok(mut file) => {
                file.write_all(result.as_bytes())?;

            }
            _ => (),
        }
    }
    Ok(())
}
