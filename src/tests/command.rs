// This method just makes sure a command runs

use crate::command::ICommand;
use crate::tests::command::add::AddCommand;
use crate::tests::data::data_storage::DataStorage;

mod add;

#[test]
fn test_command() {
  // Allocate our result storage

  let mut result_storage :DataStorage<u64> =
    DataStorage::default();

  // Allocate our add command

  let mut add_command :AddCommand =
    AddCommand::new(
      1,
      2,
      &mut result_storage
    );

  // Execute our command

  add_command.execute();

  // Make sure our command ran correctly

  assert_eq!(
    result_storage.data,
    3
  )
}