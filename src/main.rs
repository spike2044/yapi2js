use std::path::Path;

use anyhow::{anyhow, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::fs;

mod ts_template;
mod ts_types;

#[derive(Serialize, Deserialize, Debug)]
struct ReqQuery {
    required: String,
    _id: String,
    name: String,
    desc: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReqHeader {
    required: String,
    _id: String,
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReqParams {
    _id: String,
    name: String,
    desc: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReqBodyForm {
    required: String,
    _id: String,
    name: String,
    desc: Option<String>,
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq)]
enum ReqBodyType {
    #[serde(rename(deserialize = "form"))]
    Form,
    #[serde(rename(deserialize = "json"))]
    Json,
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq)]
enum ResBodyType {
    #[serde(rename(deserialize = "json"))]
    Json,
    #[serde(rename(deserialize = "raw"))]
    Raw,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YapiItem {
    path: String,
    method: String,
    title: String,
    req_body_type: ReqBodyType,
    res_body_type: ResBodyType,
    res_body: Option<String>,
    req_body_other: Option<String>,
    req_query: Vec<ReqQuery>,
    req_headers: Vec<ReqHeader>,
    req_params: Vec<ReqParams>,
    req_body_form: Vec<ReqBodyForm>,
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
struct Command {
    #[clap(short, long, required = true)]
    r#in: String,
    #[clap(short, long, required = true)]
    out_file: String,
}

async fn create_path(path: &Path) -> Result<()> {
    if fs::metadata(path).await.is_err() && !path.exists() {
        fs::create_dir_all(path).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args: Command = Command::parse();
    let in_file = &args.r#in;
    let out_file = Path::new(&args.out_file);
    let data: Vec<YapiObj> = loader(in_file).await?;
    let path = out_file
        .parent()
        .ok_or_else(|| anyhow!("out_file is not valid"))?;
    create_path(path).await?;

    ts_template::generate(out_file, &data)?;
    ts_types::generate(&data)?;
    // ts_types::generate_request(&data)?;
    Ok(())
}

async fn loader<T>(path: &str) -> Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    let str = if path.starts_with("http") || path.starts_with("https") {
        url_loader(path).await?
    } else {
        file_loader(path).await?
    };
    Ok(serde_json::from_str(&str)?)
}

async fn file_loader(path: &str) -> Result<String> {
    let in_file = Path::new(path);
    if !in_file.is_file() {
        return Err(anyhow!("in_file must file"));
    };
    let string = fs::read_to_string(in_file).await?;
    Ok(string)
}

async fn url_loader(url: &str) -> Result<String> {
    let string = reqwest::get(url).await?.text().await?;
    Ok(string)
}

