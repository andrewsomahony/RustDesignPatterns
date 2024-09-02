// Test struct to work with our chain of responsibility handlers

use std::error::Error;
use crate::chain_of_responsibility::IChainOfResponsibilityHandler;
use crate::chain_of_responsibility::list::ChainOfResponsibilityList;
use crate::tests::chain_of_responsibility::address_handler::AddressHandler;
use crate::tests::chain_of_responsibility::age_handler::AgeHandler;
use crate::tests::chain_of_responsibility::error_on_purpose_handler::ErrorOnPurposeHandler;
use crate::tests::chain_of_responsibility::name_handler::NameHandler;
use crate::tests::chain_of_responsibility::object_to_initialize::ObjectToInitialize;
use crate::tests::error::test_error::TestError;

mod object_to_initialize;
mod name_handler;
mod address_handler;
mod age_handler;
mod error_on_purpose_handler;

#[test]
fn chain_of_responsibility_success_test() {
  let name_to_use :String =
    String::from(
      "Tinus"
    );

  let age_to_use :u64 =
    105;

  let address_to_use :Vec<String> =
    vec![
      String::from("Somewhere"),
      String::from("In"),
      String::from("Delmas")
    ];

  let mut object_to_initialize =
    ObjectToInitialize::default();

  // Allocate our name handler

  let name_handler =
    Box::new(
      NameHandler::new(
        name_to_use.clone()
      )
    );

  // Allocate our address handler

  let address_handler =
    Box::new(
      AddressHandler::new(
        address_to_use.clone()
      )
    );

  // Allocate our age handler

  let age_handler  =
    Box::new(
      AgeHandler::new(
        age_to_use
      )
    );

  // We can test both chaining lists as well as multiple handlers here,
  // so we do both

  let mut first_list :ChainOfResponsibilityList<ObjectToInitialize> =
    ChainOfResponsibilityList::new();

  first_list.add_handler(
    age_handler
  );
  first_list.add_handler(
    address_handler
  );

  // Create our handler list

  let mut handler_list :ChainOfResponsibilityList<ObjectToInitialize> =
    ChainOfResponsibilityList::new();

  // Add our handlers to our handler list

  handler_list.add_handler(
    name_handler
  );
  // Add and consume our first list that we made with the address handle
  handler_list.add_list(
    first_list
  );

  // Execute our handlers on our mutable object by executing the
  // handler list

  let handler_result =
    handler_list.handle(
      &mut object_to_initialize
    );

  // Make sure our handler executed successfully

  assert_eq!(
    handler_result.is_ok(),
    true
  );

  assert_eq!(
    object_to_initialize.name,
    name_to_use
  );

  assert_eq!(
    object_to_initialize.age,
    age_to_use
  );

  // Loop through our addresses to use and make sure they are the same
  // as our object

  assert_eq!(
    object_to_initialize.address.len(),
    address_to_use.len()
  );

  for address_index in 0..address_to_use.len() {
    assert_eq!(
      object_to_initialize.address[address_index],
      address_to_use[address_index]
    );
  }
}

#[test]
fn chain_of_responsibility_fail_test() {
  let error_message_to_use :String =
    String::from(
      "Not in Delmas"
    );

  let error_handler =
    Box::new(
      ErrorOnPurposeHandler::new(
        error_message_to_use.clone()
      )
    );

  // Make a name handler

  let name_handler =
    Box::new(
      NameHandler::new(
        String::from(
          "Doesn't matter"
        )
      )
    );

  // Make an age handler

  let age_handler =
    Box::new(
      AgeHandler::new(
        99999
      )
    );

  let mut handler_list :ChainOfResponsibilityList<ObjectToInitialize> =
    ChainOfResponsibilityList::new();

  // Add our name handler
  handler_list.add_handler(
    name_handler
  );
  // Put an error handler after our name handler
  handler_list.add_handler(
    error_handler
  );
  // Put an age handler after our error handler.  We don't expect this
  // to be executed
  handler_list.add_handler(
    age_handler
  );

  // Create our mutable object to initialize

  let mut object_to_initialize :ObjectToInitialize =
    ObjectToInitialize::default();

  // Run our handler list

  let result =
    handler_list.handle(
      &mut object_to_initialize
    );

  // Make sure our result is an error

  assert_eq!(
    result.is_ok(),
    false
  );

  // Make sure our returned error is the correct type

  let returned_error:Box<dyn Error> =
    result.err().unwrap();

  assert_eq!(
    returned_error.is::<TestError>(),
    true
  );

  assert_eq!(
    returned_error.downcast_ref::<TestError>().unwrap().message,
    error_message_to_use
  );
}