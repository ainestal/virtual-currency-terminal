extern crate clap;
extern crate coinnect;

use clap::{Arg, App};
use coinnect::coinnect::Coinnect;
use coinnect::types::Pair;
use coinnect::types::Pair::*;
use coinnect::exchange::{Exchange, ExchangeApi};
use coinnect::bitstamp::BitstampCreds;
use coinnect::kraken::KrakenCreds;
use coinnect::poloniex::PoloniexCreds;

use std::process;




fn initialise_public_api(e: Exchange) -> Box<ExchangeApi> {

    match e {
        Exchange::Bitstamp => Coinnect::new(e, BitstampCreds::new("", "", "", "")).unwrap(),
        Exchange::Kraken => Coinnect::new(e, KrakenCreds::new("", "", "")).unwrap(),
        Exchange::Poloniex => Coinnect::new(e, PoloniexCreds::new("", "", "")).unwrap(),
        _ => {
            eprintln!("The given exchange, {:?}, is not supported", e);
            process::exit(1);
        },
    }
}


fn balance(e: Exchange) {
    println!("Exchange: {:?}", e)
}


fn tick(e: Exchange, p: Pair) {
    let mut api = initialise_public_api(e);
    let ticker = api.ticker(p);

    println!("{:?}: last trade price in {:?}, is {}.",
             e,
             p,
             ticker.unwrap().last_trade_price);
}

fn main() {
    let matches = App::new("Virtual Currency Terminal")
        .version("0.1")
        .author("Alejandro Inestal <ainestal@gmail.com>")
        .about("Interacts with virtual currency exchanges")
        .arg(Arg::with_name("exchange")
            .value_name("EXCHANGE")
            .help("Interacts with the given exchange. Default: Kraken")
            .required(true)
            .index(1)
            .takes_value(true))
        .arg(Arg::with_name("operation")
            .value_name("OPERATION")
            .help("Operation to apply to the exchange")
            .required(true)
            .index(2)
            .takes_value(true))
        .arg(Arg::with_name("pair")
            .value_name("PAIR")
            .help("Pair to use with the operation to apply to the exchange")
            .required(false)
            .index(3)
            .takes_value(true))
        .get_matches();


    let exchange = match matches.value_of("exchange").unwrap() {
        "kraken" => Exchange::Kraken,
        "bitstamp" => Exchange::Bitstamp,
        "poloniex" => Exchange::Poloniex,
        _ => {
            println!("Not implemented yet. Exchanges available: Bitstamp, Kraken, Poloniex");
            process::exit(1);
        }
    };
    let operation = matches.value_of("operation").unwrap();
    let pair = matches.value_of("pair").unwrap();

    println!("e: {:?}", exchange);

    println!("o: {}", operation);

    println!("p: {:?}", pair);

    let pair = match pair {
        "BTC_USD" => BTC_USD,
        _ => BTC_USD,
    };

    match operation {
        "balance" => balance(exchange),
        "tick" => tick(exchange, pair),
        _ => println!("Not yet implemented"),
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
