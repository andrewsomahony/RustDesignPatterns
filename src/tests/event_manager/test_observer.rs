use crate::event_manager::observer::IObserver;
use crate::tests::event_manager::event_object::EventObject;
use crate::tests::event_manager::event_type::EventType;

pub(super) struct TestObserver<'struct_lifetime> {
  // Our event object, whose original object has to exist
  // for at least the lifetime of the struct
  _event_object :&'struct_lifetime mut EventObject
}

impl<'struct_lifetime> TestObserver<'struct_lifetime> {
  pub(super) fn new(
    event_object :&'struct_lifetime mut EventObject
  ) -> Self {
    Self {
      _event_object: event_object
    }
  }
}

impl IObserver<'_, EventType>
  for TestObserver<'_> {
  fn notify(
    &mut self,
    data: &EventType
  ) {
    match data {
      EventType::Name(name) => {
        self._event_object.name =
          name.clone();
      }
      EventType::Payment(payment) => {
        self._event_object.payment =
          *payment;
      }
      EventType::Receipt(receipt) => {
        self._event_object.receipt_hash =
          *receipt;
      }
    }
  }
}