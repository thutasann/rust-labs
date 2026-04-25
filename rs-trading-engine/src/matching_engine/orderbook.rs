#![allow(dead_code)]

use rust_decimal::prelude::*;
use std::collections::HashMap;

use crate::matching_engine::{limit::Limit, order::Order};

#[derive(Debug)]
pub enum BidOrAsk {
  Bid,
  Ask,
}

#[derive(Debug)]
pub struct OrderBook {
  asks: HashMap<Decimal, Limit>,
  bids: HashMap<Decimal, Limit>,
}

impl OrderBook {
  pub fn new() -> OrderBook {
    OrderBook {
      asks: HashMap::new(),
      bids: HashMap::new(),
    }
  }

  pub fn fill_market_order(&mut self, market_order: &mut Order) {
    match market_order.bid_or_ask {
      BidOrAsk::Ask => {
        for limit_order in self.ask_limits() {
          limit_order.fill_order(market_order);

          if market_order.is_filled() {
            break;
          }
        }
      }
      BidOrAsk::Bid => {}
    }
  }

  // TODO: Sorting!!
  pub fn ask_limits(&mut self) -> Vec<&mut Limit> {
    return self.asks.values_mut().collect::<Vec<&mut Limit>>();
  }

  pub fn bid_limits(&mut self) -> Vec<&mut Limit> {
    return self.bids.values_mut().collect::<Vec<&mut Limit>>();
  }

  pub fn add_order(&mut self, price: Decimal, order: Order) {
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
