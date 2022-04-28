use std::collections::HashMap;
use std::fmt::{Error, format};
use std::fs;
use std::fs::OpenOptions;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera, try_get_value};
use std::io::Write;
use std::path::Path;
use serde_json::json;

use anyhow::{anyhow, Result};
use clap::{AppSettings, Parser};


#[derive(Serialize, Deserialize, Debug)]
struct YapiItem {
    path: String,
    method: String,
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct YapiObj {
    index: i32,
    name: String,
    desc: String,
    list: Vec<YapiItem>,
}

#[derive(Debug, Parser)]
#[clap(version = "0.1")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Command {
    #[clap(short, long, required = true)]
    in_file: String,
    #[clap(short, long, required = true)]
    out_file: String,
}

fn create_path(path: &Path) -> Result<(), anyhow::Error> {
    if !fs::metadata(path).is_ok() && !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn first_lower<'r, 's>(s: &'r tera::Value, _: &'s HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    let mut c = try_get_value!("data.name", "value", String, s);
    let mut c = c.chars();
    match c.next() {
        None => Ok(tera::Value::String(String::new())),
        Some(f) => Ok(tera::Value::String(f.to_lowercase().collect::<String>() + c.as_str())),
    }
}

fn main() -> Result<(), anyhow::Error> {
    let temp = r#"import { apiConfig } from 'utils/api'
{% for data in list %}
export const {{ data.name | first_lower }} = apiConfig({
{% for item in data.list %}
  {{ item.title }}: [
  '{{item.method}}',
  '{{item.path}}'
  ],
{% endfor %}
})
{% endfor %}
"#;

    let args: Command = Command::parse();
    let in_file = Path::new(&args.in_file);
    let out_file = Path::new(&args.out_file);


    if !in_file.is_file() {
        panic!("in_file must file");
    }

    let string = fs::read_to_string(in_file)?;
    let data = serde_json::from_str::<Vec<YapiObj>>(&string)?;

    let path = out_file.parent().ok_or(anyhow!("out_file is not valid"))?;
    create_path(path)?;

    let mut tera = Tera::default();
    tera.register_filter("first_lower", first_lower);
    tera.add_raw_template("api", temp)?;
    let mut context = Context::new();
    context.insert("list", &data);

    match OpenOptions::new().create(true).read(true).write(true).open(out_file) {
        Ok(mut file) => {
            let code = tera.render("api", &context)?;
            // println!("{}", code);
            file.write_all(code.as_bytes())?;
            Ok(())
        }
        Err(e) => {
            println!("{}", e);
            Err(anyhow!(e.to_string()))
        }
    }
}
