use std::{thread, time};
use std::env;
use reqwest::blocking::Client;
use anyhow::*;

use serde::{Deserialize, Serialize};

// {"id":1,"customerId":1,"amount":{"value":70.03,"currency":"USD"},"status":"PAID"}
#[derive(Debug, Serialize, Deserialize)]
struct Invoice {
    id: i32,
    #[serde(rename = "customerId")]
    customer_id: i32,
    status: String,
}

fn invoices() -> Result<Vec<Invoice>> {
    let invoice = env::var("INVOICEURL").unwrap_or("http://localhost:8000/rest/v1/invoices".to_string());

    let client = Client::new();
    client.get(invoice.clone())
        .send().with_context(|| format!("GET to {} failed", invoice.clone()))?
        .json::<Vec<Invoice>>().context("failed to parse invoices response")
}

fn pay() -> Result<bool> {
    let pay_url = env::var("PAYURL").unwrap_or("http://localhost:8000/rest/v1/invoices/pay".to_string());

    let client = Client::new();
    client.post(pay_url.clone())
        .send().with_context(|| format!("POST to {} failed", pay_url.clone()))?
        .json::<bool>().context("failed to parse pay response")
}

fn retry<T, F: Fn() -> Result<T> > (max_retries: u64, f: F) -> Result<T> {
    let res = f();
    match res {
        Ok(_) => {
            res
        }
        Err(err) => {
            if max_retries <= 0 {
                Err(err)
            } else {
                let next_retries = max_retries - 1;
                println!("{} retries left", (next_retries).to_string());
                let ms = time::Duration::from_millis(10000);
                println!("sleeping for {:?}", ms);
                thread::sleep(ms);
                retry(next_retries, f)
            }
        }
    }
}

fn count_paid(invs : Vec<Invoice>) -> usize {
    invs.iter().filter(|&x| x.status == "PAID").count()
}

fn check_invoices(invs : Vec<Invoice>) -> Result<usize> {
    let paid = count_paid(invs);
    if paid > 2 { // There are 2 paid invoices in the db already
        Ok(paid)
    } else {
        Err(anyhow!("No invoices were paid"))
    }
}

fn test_pay() -> Result<()> {
    let max_retries = env::var("MAX_RETRIES").unwrap_or("".to_string());
    let retry_count = max_retries.parse::<u64>().unwrap_or(30);

    let orig_invs = retry(retry_count, invoices).context("Initial get to invoices failed")?;
    println!("Intial count of paid invoices: {:#?}", count_paid(orig_invs));

    let paid = retry(retry_count, pay).context("Attempt to pay invoices failed")?;
    if paid {
        println!("All invoices paid, response: {:#?}", paid);
        Ok(())
    } else {
        let inv = retry(retry_count, invoices).context("Failed to get updated invoices")?;
        let new_count = check_invoices(inv)?;
        println!("Amount of paid invoices: {:#?}", new_count);
        Ok(())
    }
}

fn main() -> Result<()> {
    std::process::exit(match test_pay() {
        Ok(()) => 0,
        Err(err) => {
            println!("{:#?}", err);
            1
        }
    });
}
