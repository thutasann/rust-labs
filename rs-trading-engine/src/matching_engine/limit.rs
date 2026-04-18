use crate::matching_engine::{order::Order, price::Price};

#[derive(Debug)]
pub struct Limit {
  price: Price,
  orders: Vec<Order>,
}

impl Limit {
  pub fn new(price: Price) -> Limit {
    Limit {
      price: price,
      orders: Vec::new(),
    }
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
    }
  }

  pub fn add_order(&mut self, order: Order) {
    self.orders.push(order);
  }
}
