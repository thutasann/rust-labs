#[cfg(test)]
pub mod tests {
  use rust_decimal_macros::dec;

  use crate::matching_engine::{limit::Limit, order::Order, orderbook::BidOrAsk};

  #[test]
  fn limit_total_volume() {
    let price = dec!(10000);
    let mut limit = Limit::new(price);
    let buy_limit_order_a = Order::new(BidOrAsk::Bid, 100.0);
    let buy_limit_order_b = Order::new(BidOrAsk::Bid, 100.0);

    limit.add_order(buy_limit_order_a);
    limit.add_order(buy_limit_order_b);

    assert_eq!(limit.total_volume(), 200.0)
  }

  #[test]
  fn limit_order_multi_fill() {
    let price = dec!(10000);
    let mut limit = Limit::new(price);
    let buy_limit_order_a = Order::new(BidOrAsk::Bid, 100.0);
    let buy_limit_order_b = Order::new(BidOrAsk::Bid, 100.0);
    limit.add_order(buy_limit_order_a);
    limit.add_order(buy_limit_order_b);

    let mut market_sell_order = Order::new(BidOrAsk::Ask, 199.0);
    limit.fill_order(&mut market_sell_order);

    assert_eq!(market_sell_order.is_filled(), true);
    assert_eq!(limit.orders.get(0).unwrap().is_filled(), true);
    assert_eq!(limit.orders.get(1).unwrap().is_filled(), false);
    assert_eq!(limit.orders.get(1).unwrap().size, 1.0);

    println!("{:?}", limit)
  }

  #[test]
  fn limit_order_single_fill() {
    let price = dec!(10000);
    let mut limit = Limit::new(price);
    let buy_limit_order = Order::new(BidOrAsk::Bid, 100.0);
    limit.add_order(buy_limit_order);

    let mut market_sell_order = Order::new(BidOrAsk::Ask, 99.0);
    limit.fill_order(&mut market_sell_order);

    assert_eq!(market_sell_order.is_filled(), true);
    assert_eq!(limit.orders.get(0).unwrap().size, 1.0);
  }
}
