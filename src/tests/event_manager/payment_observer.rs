use std::cell::RefCell;
use crate::event_manager::observer::IObserver;
use crate::tests::event_manager::event_object::EventObject;
use crate::tests::event_manager::event_type::EventType;

// This observer observes changes to the payment, and sets
// the shared event object's payment accordingly

pub (super) struct PaymentObserver<'struct_lifetime> {
  _event_object :&'struct_lifetime RefCell<EventObject>
}

impl<'struct_lifetime> PaymentObserver<'struct_lifetime> {
  pub(super) fn new(
    event_object :&'struct_lifetime RefCell<EventObject>
  ) -> Self {
    Self {
      _event_object: event_object
    }
  }
}

impl IObserver<EventType>
  for PaymentObserver<'_> {
  fn notify(
    &mut self,
    data: &EventType
  ) {
    match data {
      EventType::Payment(data) => {
        // Set our payment value

        self._event_object.borrow_mut().payment =
          *data;
      }
      _ => {
        // Do nothing for the other cases
      }
    }
  }
}