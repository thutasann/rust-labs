use std::collections::HashMap;

use crate::matching_engine::{limit::Limit, order::Order, price::Price};

#[derive(Debug)]
pub enum BidOrAsk {
  Bid,
  Ask,
}

#[derive(Debug)]
pub struct OrderBook {
  asks: HashMap<Price, Limit>,
  bids: HashMap<Price, Limit>,
}

impl OrderBook {
  pub fn new() -> OrderBook {
    OrderBook {
      asks: HashMap::new(),
      bids: HashMap::new(),
    }
  }

  pub fn add_order(&mut self, price: f64, order: Order) {
    let price = Price::new(price);

    match order.bid_or_ask {
      BidOrAsk::Bid => match self.bids.get_mut(&price) {
        Some(limit) => {
          limit.add_order(order);
        }
        None => {
          let mut limit = Limit::new(price);
          limit.add_order(order);
          self.bids.insert(price, limit);
        }
      },
      BidOrAsk::Ask => match self.bids.get_mut(&price) {
        Some(limit) => {
          limit.add_order(order);
        }
        None => {
          let mut limit = Limit::new(price);
          limit.add_order(order);
          self.asks.insert(price, limit);
        }
      },
    }
  }
}
