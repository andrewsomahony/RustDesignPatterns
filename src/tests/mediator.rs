use crate::mediator::IMediator;
use crate::tests::data::data_storage::DataStorage;
use crate::tests::mediator::test_data_mediator::TestDataMediator;

mod test_data_mediator;

#[test]
fn mediator_test() {
  // Allocate our data storage

  let mut data_storage :DataStorage<u64> =
    DataStorage::default();

  // Allocate our mediator

  let mut test_data_mediator :TestDataMediator =
    TestDataMediator::new(
      // Note: We can use a mutable reference here as the compiler
      // has figured out that we aren't using it after we execute
      &mut data_storage
    );

  // Execute our mediator with our input value

  test_data_mediator.execute(
    10
  );

  // Assert that our data storage's data value
  // is what we expect

  assert_eq!(
    data_storage.data,
    20
  )
}