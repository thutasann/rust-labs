use std::{collections::HashMap, hash::Hash};

use crate::matching_engine::orderbook::OrderBook;

// BTCUSD
// BTC => BASE
// USD => QUOTE
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
  base: String,
  quote: String,
}

impl TradingPair {
  pub fn new(base: String, quote: String) -> TradingPair {
    TradingPair { base, quote }
  }
}

pub struct MatchingEngine {
  orderbooks: HashMap<TradingPair, OrderBook>,
}

impl MatchingEngine {
  pub fn new() -> MatchingEngine {
    MatchingEngine {
      orderbooks: HashMap::new(),
    }
  }

  pub fn add_new_market(&mut self, pair: TradingPair) {
    self.orderbooks.insert(pair.clone(), OrderBook::new());
    println!("opening new orderbook for market {:?}", pair)
  }
}
