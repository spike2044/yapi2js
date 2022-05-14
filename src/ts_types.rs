use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Write;
use std::string::String;

use anyhow::{anyhow, Result as AnyResult};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{ReqBodyType, ResBodyType, YapiObj};

// use serde_json::Value::String;
// use serde_json::Value::String;

// use serde_json::Value::String;

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
            let body = match item.res_body_type {
                ResBodyType::json => { &item.res_body }
                ResBodyType::raw => { continue; }
            };
            if !re.is_match(&item.title) || body.is_none() || body == &Some(String::from("")) {
                continue;
            }
            let body = body.as_ref().ok_or_else(|| anyhow!("no res_body"))?;
            let v: ResponseValueType = serde_json::from_str(body)?;
            let title = format!("{}{}", obj.name, item.title);
            if !set.contains(&title) {
                result = format!("{} \n export type {}{}Type = {}", result, obj.name, item.title, get_response_type(&v));
                set.insert(title);
            }
        }
    }
    if let Ok(mut file) = OpenOptions::new().read(true).write(true).truncate(true).create(true).open("type.ts") {
        file.write_all(result.as_bytes())?;
    }
    Ok(())
}

pub fn generate_request(data: &Vec<YapiObj>) -> AnyResult<()> {
    let re = Regex::new(r"^\w+$").unwrap();
    let mut set = HashSet::new();
    let mut result = String::new();
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("text"), String::from("string"));
    map.insert(String::from("file"), String::from("File"));
    map.insert(String::from("number"), String::from("number"));
    for obj in data {
        for item in &obj.list {
            let title = format!("{}RequestType", item.title);
            if !re.is_match(&item.title) || set.contains(&item.title) {
                continue;
            }
            set.insert(&item.title);

            let mut req_params = item.req_params.iter().map(|x| format!("{}: string\n", x.name)).collect::<String>();
            if !req_params.is_empty() {
                req_params = format!("\n uri: {{\n{} }}", req_params);
            }

            let mut req_query = item.req_query.iter().map(|x| format!("{}{}: string\n", x.name, if x.required == "1" { "" } else { "?" })).collect::<String>();
            if !req_query.is_empty() {
                req_query = format!("\nparams: {{\n{} }}", req_query);
            }

            let mut data = match item.req_body_type {
                ReqBodyType::form => {
                    item.req_body_form.iter().map(|x| format!("{}?: {} \n", x.name, map.get(&x.name).cloned().unwrap_or("unknown".to_string()))).collect::<String>()
                }
                ReqBodyType::json => {
                    match item.req_body_other {
                        Some(ref body) => {
                            let v: ResponseValueType = serde_json::from_str(body)?;
                            get_response_type(&v).trim_matches(|c| c == '{' || c == '}').to_string()
                        }
                        None => String::from("")
                    }
                }
                _ => { String::from("") }
            };
            if !data.is_empty() {
                data = format!("\ndata: {{\n{} }}", data);
            };

            result = format!("{} \n export type {} = {{ {}{}{}\n }}", result, title, req_params, req_query, data);
        }
    }
    if let Ok(mut f) = OpenOptions::new().read(true).write(true).truncate(true).create(true).open("request.ts") {
        f.write_all(result.as_bytes())?;
    }
    Ok(())
}


