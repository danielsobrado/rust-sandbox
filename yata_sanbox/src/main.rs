use yata::helpers::{RandomCandles, MA};
use yata::indicators::MACD;
use yata::prelude::*;

fn main() {
    let mut candles = RandomCandles::new();
    let mut macd = MACD::default();
    
    macd.ma1 = "sma-4".parse().unwrap(); // one way of defining methods inside indicators
    
    macd.signal = MA::TEMA(5); // another way of defining methods inside indicators
    
    let mut macd = macd.init(&candles.first()).unwrap();
    
    for candle in candles.take(10) {
        let result = macd.next(&candle);
    
        println!("{:?}", result);
    }
}
