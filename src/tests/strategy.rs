use crate::strategy::IStrategy;
use crate::tests::strategy::test_strategy::TestStrategy;

mod test_strategy;

#[test]
fn strategy_test() {
  // Allocate our strategy

  let test_value :u64 =
    5;

  let mut test_strategy :TestStrategy =
    TestStrategy::new(
      &test_value
    );

  assert_eq!(
    test_strategy.execute(
      10
    ),
    false
  )
}