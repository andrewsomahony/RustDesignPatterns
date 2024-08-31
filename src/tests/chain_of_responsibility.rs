// Test struct to work with our chain of responsibility handlers

use crate::chain_of_responsibility::IChainOfResponsibilityHandler;
use crate::chain_of_responsibility::list::ChainOfResponsibilityList;
use crate::tests::chain_of_responsibility::address_handler::AddressHandler;
use crate::tests::chain_of_responsibility::age_handler::AgeHandler;
use crate::tests::chain_of_responsibility::name_handler::NameHandler;
use crate::tests::chain_of_responsibility::object_to_initialize::ObjectToInitialize;

mod object_to_initialize;
mod name_handler;
mod address_handler;
mod age_handler;

#[test]
fn chain_of_responsibility_test() {
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

  let mut object_to_initialize :ObjectToInitialize =
    ObjectToInitialize::new();

  // Allocate our name handler

  let name_handler: Box<dyn IChainOfResponsibilityHandler<ObjectToInitialize>> =
    Box::new(
      NameHandler::new(
        name_to_use.clone()
      )
    );

  // Allocate our address handler

  let address_handler :Box<dyn IChainOfResponsibilityHandler<ObjectToInitialize>> =
    Box::new(
      AddressHandler::new(
        address_to_use.clone()
      )
    );

  // Allocate our age handler

  let age_handler :Box<dyn IChainOfResponsibilityHandler<ObjectToInitialize>> =
    Box::new(
      AgeHandler::new(
        age_to_use
      )
    );

  // Create our handler list

  let mut handler_list :ChainOfResponsibilityList<ObjectToInitialize> =
    ChainOfResponsibilityList::new();

  // Add our handlers to our handler list

  handler_list.add_handler(
    name_handler
  );
  handler_list.add_handler(
    age_handler
  );
  handler_list.add_handler(
    address_handler
  );

  // Execute our handlers on our mutable object by executing the
  // handler list

  handler_list.handle(
    &mut object_to_initialize
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