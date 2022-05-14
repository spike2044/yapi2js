use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::prelude::*;

use anyhow::{anyhow, Result as AnyResult};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::YapiObj;

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
                    let _ = writeln!(result, "{}:{}", k, get_response_type(v));
                }
                result.push('}');
                return result;
            }
            String::from("{}")
        }
        "array" => {
            if let Some(items) = &value.items {
                return format!("{}[]", get_response_type(items));
            }
            String::new()
        }
        "string" => {
            String::from("string")
        }
        "integer" => {
            String::from("number")
        }
        "boolean" => {
            String::from("boolean")
        }
        _ => String::from("unknown")
    }
}

pub fn generate(data: &Vec<YapiObj>) -> AnyResult<()> {
    let re = Regex::new(r"^\w+$").unwrap();
    let mut set = HashSet::new();
    let mut result = String::new();
    for obj in data {
        for item in &obj.list {
            if item.res_body_type != "json" || !re.is_match(&item.title) || item.res_body.is_none() {
                continue;
            }
            let body = item.res_body.as_ref().ok_or_else(|| anyhow!("no res_body"))?;
            let mut v: ResponseValueType = serde_json::from_str(&body)?;
            let title = format!("{}{}", obj.name, item.title);
            if !set.contains(&title) {
                result = format!("{} \n export type {}{} = {}", result, obj.name, item.title, get_response_type(&v));
                set.insert(title);
            }
        }
    }
    if let Ok(mut file) = OpenOptions::new().read(true).write(true).truncate(true).create(true).open("type.ts") {
        file.write_all(result.as_bytes())?;
    }
    Ok(())
}
