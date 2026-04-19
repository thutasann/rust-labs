use crate::matching_engine::{order::Order, price::Price};

#[derive(Debug)]
pub struct Limit {
  pub price: Price,
  pub orders: Vec<Order>,
}

impl Limit {
  pub fn new(price: Price) -> Limit {
    Limit {
      price: price,
      orders: Vec::new(),
    }
  }

  pub fn total_volume(&self) -> f64 {
    return self
      .orders
      .iter()
      .map(|order| order.size)
      .reduce(|a, b| a + b)
      .unwrap();
  }

  pub fn fill_order(&mut self, market_order: &mut Order) {
    for limit_order in self.orders.iter_mut() {
      match market_order.size >= limit_order.size {
        true => {
          market_order.size -= limit_order.size;
          limit_order.size = 0.0;
        }
        false => {
          limit_order.size -= market_order.size;
          market_order.size = 0.0;
        }
      }

      if market_order.is_filled() {
        break;
      }
    }
  }

  pub fn add_order(&mut self, order: Order) {
    self.orders.push(order);
  }
}
