#[cfg(test)]
pub mod tests {
  use rust_decimal_macros::dec;

  use crate::matching_engine::{
    limit::Limit,
    order::Order,
    orderbook::{BidOrAsk, OrderBook},
  };

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

  #[test]
  fn orderbook_fill_market_order_ask() {
    let mut orderbook = OrderBook::new();
    orderbook.add_order(dec!(500), Order::new(BidOrAsk::Ask, 10.0));
    orderbook.add_order(dec!(100), Order::new(BidOrAsk::Ask, 10.0));
    orderbook.add_order(dec!(200), Order::new(BidOrAsk::Ask, 10.0));
    orderbook.add_order(dec!(300), Order::new(BidOrAsk::Ask, 10.0));

    let mut market_order = Order::new(BidOrAsk::Ask, 10.0);
    orderbook.fill_market_order(&mut market_order);

    let ask_limits = orderbook.ask_limits();
    let matched_limit = ask_limits.get(0).unwrap();
    assert_eq!(matched_limit.price, dec!(100));
    assert_eq!(market_order.is_filled(), true);

    let matched_order = matched_limit.orders.get(0).unwrap();
    assert_eq!(matched_order.is_filled(), true);
  }
}
