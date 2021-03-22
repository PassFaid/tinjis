use std::{thread, time};
use std::env;
use reqwest::blocking::Client;
use std::error::Error;
use serde::{Deserialize, Serialize};

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

// {"id":1,"customerId":1,"amount":{"value":70.03,"currency":"USD"},"status":"PAID"}
#[derive(Debug, Serialize, Deserialize)]
struct Invoice {
    id: i32,
    #[serde(rename = "customerId")]
    customer_id: i32,
    status: String,
}

fn invoices() -> Result<Vec<Invoice>, Box<dyn std::error::Error>> {
    let pay_url = env::var("INVOICEURL").unwrap_or("http://localhost:8000/rest/v1/invoices".to_string());

    let client = Client::new();
    let res = client.get(pay_url)
        .send();

    match res {
        Ok(_res) => {
            match _res.json::<Vec<Invoice>>() {
                Ok(inv) => {
                    Ok(inv)
                }
                Err(err) => {
                    println!("{:#?}", err.to_string());
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

fn pay() -> Result<bool, Box<dyn std::error::Error>> {
    let pay_url = env::var("PAYURL").unwrap_or("http://localhost:8000/rest/v1/invoices/pay".to_string());

    let client = Client::new();
    let res = client.post(pay_url)
        .send();

    match res {
        Ok(_res) => {
            match _res.json::<bool>() {
                Ok(_bool) => {
                    Ok(_bool)
                }
                Err(_err) => {
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
    let max_retries = 30;
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

fn count_paid(invs : Vec<Invoice>) -> usize {
    invs.iter().filter(|&x| x.status == "PAID").count()
}

fn check_invoices(invs : Vec<Invoice>) -> Result<usize, String> {
    let paid = count_paid(invs);
    if paid > 2 {
        Ok(paid)
    } else {
        Err("No invoices were paid".to_string())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::process::exit(match retry() {
        Ok(paid) => {
            if paid {
                println!("All invoices paid, response: {:#?}", paid);
                0
            } else {
                let inv = invoices()?;
                let res = check_invoices(inv);
                match res {
                    Ok(paid_count) => {
                        println!("Amount of paid invoices: {:#?}", paid_count);
                        0
                    }
                    Err(err) => {
                        println!("{:#?}", err);
                        1
                    }
                }
            }
        }
        Err(err) => {
            println!("{:#?}", err);
            1
        }
    });
}
