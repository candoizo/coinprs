macro_rules! aw {
  ($e:expr) => {
      tokio_test::block_on($e)
  };
}

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[test]
fn tests_can_run() {
    assert_eq!(2 + 2, 4);
}

#[path = "../src/config.rs"]
mod conf;

#[path = "../src/table.rs"]
mod table;
mod config {
    use super::*;
    #[test]
    fn default_config() {
        let _conf = conf::get_config();
    }


    use comfy_table::CellAlignment;
    #[test]
    fn parse_align() {
        let success_values = vec![
            "center",
            "CenTer",
            "CENTER",
            "C",
            "c",
            "l",
            "L",
            "r",
            "R",
            "rightleft",
            "rleft", // returns right
        ];
        // should return dynamic values
        let success_values: Vec<CellAlignment> = success_values
            .iter()
            .map(|x| conf::parse_align(&x.to_string()))
            .collect();

        // should return defaults
        let fail_values = vec!["", "#", "aasdasdas"];
        let fail_values: Vec<CellAlignment> =
            fail_values.iter().map(|x| conf::parse_align(x)).collect();
    }

    #[test]
    fn parse_tint() {
        // should return dynamic values
        let success_values = vec!["red", "RED", "rEd", "blue", "#FF0000", "FF0000"];
        let success_values: Vec<comfy_table::Color> = success_values
            .iter()
            .map(|x| conf::parse_tint(&x.to_string()))
            .collect();

        // should return default values
        let fail_values = vec!["avsjfsdh", "", "rred", "re d", "#", "xzcsadewksgnsdjfn"];
        let fail_values: Vec<comfy_table::Color> = fail_values
            .iter()
            .map(|x| conf::parse_tint(&x.to_string()))
            .collect();
    }
}

#[path = "../src/currency.rs"]
mod currency;

use coingecko::{Client, SimplePriceReq};

// mod tests {
//     use super::*;
//     use coingecko::{Client, SimplePriceReq};
//
//     #[test]
//     fn to_asset() {
//         let http = isahc::HttpClient::new().unwrap();
//         let client = Client::new(http);
//
//         let test_asset = "bitcoin";
//
//         let req = SimplePriceReq::new(test_asset.to_owned(), "USD".to_owned())
//             .include_market_cap()
//             .include_24hr_vol()
//             .include_24hr_change()
//             .include_last_updated_at();
//         let res = aw!(client.simple_price(req)).unwrap();
//         let curr = currency::to_coingecko_response(&res);
//         assert_eq!(curr.asset, test_asset);
//
//         // let asset: currency::Asset = currency::to_asset(curr);
//         // futures::future::await(client.simple_price(req));
//     }
// }


mod money {
    use super::*;

    #[test]
    fn to_shortnum() {
        let mut num: f64 = 0.0;
        assert_eq!("0", currency::to_shortnum(num));
        num += 1.5;
        assert_eq!("1.5", currency::to_shortnum(num));
        num += 1e3;
        assert_eq!("1001.5", currency::to_shortnum(num));
        num += 1e6;
        assert_eq!("1M", currency::to_shortnum(num));
        num += 1e6;
        assert_eq!("2M", currency::to_shortnum(num));
        num += 3e5 + 1.0; // cus it rounds to like 2.29999999999999999999_
        assert_eq!("2.3M", currency::to_shortnum(num));
        num += 2e7 + 1.0; // cus it rounds to like 2.29999999999999999999_
        assert_eq!("22.3M", currency::to_shortnum(num));
        num += 1e9; // cus it rounds to like 2.29999999999999999999_
        assert_eq!("1.02B", currency::to_shortnum(num));
        num += 1e11;
        assert_eq!("101.02B", currency::to_shortnum(num));
    }

    #[test]
    fn to_currency() {
        let success_values = vec!["usd", "USD", "Usd", "blue", "#FF0000", "FF0000"];
        // @todo make a parse currency function?
        // also make on efor converting from usd -> desired currency
        let success_values: Vec<comfy_table::Color> = success_values
            .iter()
            .map(|x| conf::parse_tint(&x.to_string()))
            .collect();
    }

    // use coingecko::{Client, SimplePriceReq};
    #[test]
    fn to_asset() {
        let http = isahc::HttpClient::new().unwrap();
        let client = Client::new(http);

        let test_asset = "bitcoin";

        let req = SimplePriceReq::new(test_asset.to_owned(), "USD".to_owned())
            .include_market_cap()
            .include_24hr_vol()
            .include_24hr_change()
            .include_last_updated_at();
        let res = aw!(client.simple_price(req)).unwrap();
        let curr = currency::to_coingecko_response(&res);
        assert_eq!(curr.asset, test_asset);

        // let asset: currency::Asset = currency::to_asset(curr);
        // futures::future::await(client.simple_price(req));
    }

    #[test]
    fn to_coingecko_response() {
        let http = isahc::HttpClient::new().unwrap();
        let client = Client::new(http);

        let test_asset = "bitcoin";

        let req = SimplePriceReq::new(test_asset.to_owned(), "USD".to_owned())
            .include_market_cap()
            .include_24hr_vol()
            .include_24hr_change()
            .include_last_updated_at();
        let res = aw!(client.simple_price(req)).unwrap();
        let curr = currency::to_coingecko_response(&res);
        assert_eq!(curr.asset, test_asset)
        // futures::future::await(client.simple_price(req));
    }


}
