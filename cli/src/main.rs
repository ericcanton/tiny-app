use std::env;
use std::process;
use std::io::Read;

use reqwest;
use colored::*;

use serde_json;
use serde_derive::{Serialize, Deserialize};

use error_chain::{error_chain, bail};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Serialize, Deserialize)]
struct Args {
    a: f32,
    b: f32
}

impl Args {
    fn new(args: &[String]) -> Result<Args> {
        if args.len() != 3 {
            bail!("Call with exactly 2 arguments.");
        }

        let a_arg = args[1].parse::<f32>();
        let b_arg = args[2].parse::<f32>();

        Ok(Args{
            a: a_arg.unwrap(),
            b: b_arg.unwrap()
        })
    }
}

// Needed so we can use .body on the reqwest::blocking::Client below
impl From<Args> for reqwest::blocking::Body {
    fn from(args: Args) -> Self {
        reqwest::blocking::Body::from(
            serde_json::to_string(&args).unwrap()
        )
    }
}


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let req_body = Args::new(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
    });

    let client = reqwest::blocking::Client::new();

    let mut is_json = reqwest::header::HeaderMap::new();

    is_json.insert(reqwest::header::CONTENT_TYPE, "application/json".parse().unwrap());

    let mut res = client.post("https://<insert-id>.execute-api.us-east-1.amazonaws.com/prod/ab")
        .headers(is_json)
        .body(req_body)
        .send()?;

    
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body.green());

    Ok(())
}