use coingecko::{Client, SimplePriceReq, SimplePrices};
use config::Value;
use isahc::HttpClient;
pub async fn get_price(http: HttpClient, i: Value) -> Result<SimplePrices, coingecko::Error> {
    let t = i.into_table().unwrap();
    let client = Client::new(http.to_owned());
    let name: String = t["name"].to_string();
    let req = SimplePriceReq::new(name, "usd".into())
        .include_market_cap()
        .include_24hr_vol()
        .include_24hr_change()
        .include_last_updated_at();
    Ok(client.simple_price(req).await.unwrap())
}

use rusty_money::iso::Currency;
use rusty_money::{iso, Money};
use rust_decimal::Decimal;
pub fn to_fiat(x: Decimal, fiat: &'static Currency) -> Money<'static, iso::Currency> {
    Money::from_decimal(x, *&fiat)
}
