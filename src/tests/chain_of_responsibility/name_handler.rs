use std::error::Error;
use crate::chain_of_responsibility::IChainOfResponsibilityHandler;
use crate::tests::chain_of_responsibility::object_to_initialize::ObjectToInitialize;

pub(super) struct NameHandler {
  // We need ownership of this string, as this handler needs
  // to exist on its own and not bound to a string value somewhere
  _name_to_use :String
}

impl NameHandler {
  pub fn new(
    name_to_use :String
  ) -> Self {
    Self {
      _name_to_use: name_to_use
    }
  }
}

impl IChainOfResponsibilityHandler<ObjectToInitialize>
  for NameHandler {
  fn handle(
    &self,
    object: &mut ObjectToInitialize
  ) -> Result<(), Box<dyn Error>> {
    // Set our object's name, give it its own copy of the name
    object.name =
      self._name_to_use.clone();

    return Ok(());
  }
}