use std::collections::HashMap;
use std::fs::OpenOptions;
use tera::{Context, Tera, try_get_value};
use std::io::Write;
use std::path::Path;

use anyhow::{anyhow, Result};
use crate::YapiObj;


fn first_lower(s: &tera::Value, _: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    let c = try_get_value!("data.name", "value", String, s);
    let mut c = c.chars();
    match c.next() {
        None => Ok(tera::Value::String(String::new())),
        Some(f) => Ok(tera::Value::String(f.to_lowercase().collect::<String>() + c.as_str())),
    }
}

fn lower_case(s: &tera::Value, _: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    Ok(tera::Value::String(try_get_value!("data", "value", String, s).to_lowercase()))
}


pub fn generate(out_file: &Path, data: &Vec<YapiObj>) -> Result<()> {
    let temp = r#"import { apiConfig } from 'utils/api'
{% for data in list %}
export const {{ data.name | first_lower }} = apiConfig({
{% for item in data.list %}
  {{ item.title }}: [
  '{{item.method | lower }}',
  '{{item.path}}'
  ],
{% endfor %}
})
{% endfor %}
"#;
    let mut tera = Tera::default();
    tera.register_filter("first_lower", first_lower);
    tera.register_filter("lower", lower_case);
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
