use std::{thread, time};
use std::env;
use reqwest::blocking::Client;
use std::error::Error;

#[macro_use]
extern crate simple_error;

#[derive(Debug, Default)]
struct UnknownErr;

impl std::fmt::Display for UnknownErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("ERROR")
    }
}
impl Error for UnknownErr {}

fn pay() -> Result<bool, Box<dyn std::error::Error>> {
    let pay_url = env::var("PAYURL").unwrap_or("http://localhost:8000/rest/v1/invoices/pay".to_string());

    let client = Client::new();
    let res = client.post(pay_url)
        .send();

    match res {
        Ok(_res) => {
            println!("{:#?}", _res);
            match _res.json::<bool>() {
                Ok(_bool) => {
                    Ok(_bool)
                }
                Err(_err) => {
                    println!("{:#?}", _err.to_string());
                    bail!("couldn't parse response");
                }
            }
        }
        Err(err) => {
            println!("{:#?}", err.to_string());
            bail!("pay request failed");
        }
    }
}

fn retry() -> Result<bool, Box<dyn std::error::Error>> {
    let max_retries = 5;
    let mut res : Result<bool, Box<dyn std::error::Error>> = Err(Box::new(UnknownErr));
    for i in 1..max_retries {
        let res_ = pay();
        match res_ {
            Ok(_bool) => {
                res = Ok(_bool);
                break;
            }
            Err(err) => {
                println!("{} retries left", (max_retries - i).to_string());
                let ms = time::Duration::from_millis(1000*i);
                println!("sleeping for {:?}", ms);
                thread::sleep(ms);
                res = Err(err)
            }
        }
    }
    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::process::exit(match retry() {
        Ok(bool) => {
            println!("{:#?}", bool);
            0
        }
        Err(err) => {
            println!("{:#?}", err);
            1
        }
    });
}
