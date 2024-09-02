use std::error::Error;
use crate::chain_of_responsibility::IChainOfResponsibilityHandler;
use crate::tests::chain_of_responsibility::object_to_initialize::ObjectToInitialize;

pub(super) struct AddressHandler {
  // We need our address to use to have its own ownership of
  // the vector, to avoid being bound by another value somewhere whose
  // lifetime may not be as long as our handler needs to live
  _address_to_use :Vec<String>
}

impl AddressHandler {
  pub(super) fn new(
    address_to_use :Vec<String>
  ) -> Self {
    Self {
      _address_to_use: address_to_use
    }
  }
}

impl IChainOfResponsibilityHandler<'_, ObjectToInitialize>
  for AddressHandler {
  fn handle(
    &self,
    object: &mut ObjectToInitialize
  ) -> Result<(), Box<dyn Error>> {
    // Give the object its own copy of the address vector
    object.address =
      self._address_to_use.clone();

    return Ok(());
  }
}