use crate::matching_engine::orderbook::BidOrAsk;

#[derive(Debug)]
pub struct Order {
  pub size: f64,
  pub bid_or_ask: BidOrAsk,
}

impl Order {
  pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
    Order { size, bid_or_ask }
  }

  pub fn is_filled(&self) -> bool {
    self.size == 0.0
  }
}
