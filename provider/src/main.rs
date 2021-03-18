use tide::Request;
use tide::prelude::*;
use tide::log::LevelFilter::*;

#[derive(Debug, Deserialize)]
struct Currency {
}
#[derive(Debug, Deserialize)]
struct Invoice {
    currency: Currency,
    customer_id: u8,
    value: f32,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::with_level(Info);
    let mut app = tide::new();
    app.at("/api/pay").post(pay_invoice);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn pay_invoice(mut req: Request<()>) -> tide::Result {
    let inv : Invoice = req.body_json().await?;
    tide::log::info!("paying {:?} invoice in {:?} for amount {:?}", inv.currency, inv.customer_id, inv.value);
    Ok(json!(true).into())
}
