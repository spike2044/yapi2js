use std::fs;
use std::fs::OpenOptions;
use serde::{Deserialize, Serialize};
use std::path::Path;
use serde_json::json;

use anyhow::{anyhow, Result};
use clap::{AppSettings, Parser};


mod ts_types;
mod ts_template;

use ts_template::*;
use ts_types::*;


#[derive(Serialize, Deserialize, Debug)]
struct ReqQuery {
    required: String,
    _id: String,
    name: String,
    desc: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct YapiItem {
    path: String,
    method: String,
    title: String,
    res_body_type: String,
    res_body: Option<String>,
    req_query: Option<Vec<ReqQuery>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YapiObj {
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



fn main() -> Result<(), anyhow::Error> {


    // ResponseValue

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

    ts_template::generate(out_file, &data)?;
    ts_types::generate( &data)?;
    Ok(())
}
