use reqwest;
use colored::*;

use std::io::Read;
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}


fn main() -> Result<()> {
    let mut res = reqwest::blocking::get(
        "https://wj2o3qsbc8.execute-api.us-east-1.amazonaws.com/prod/yo?name=eric"
    )?;
    
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body.green());

    Ok(())
}