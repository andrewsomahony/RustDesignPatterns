use std::cell::{RefCell};
use crate::event_manager::observer::IObserver;
use crate::tests::event_manager::event_object::EventObject;
use crate::tests::event_manager::event_type::EventType;

// This observer observes changes to the name, and sets
// the shared event object's name accordingly

pub(super) struct NameObserver<'struct_lifetime> {
  // Our event object, whose original object has to exist
  // for at least the lifetime of the struct

  // Our event object is shared state, so we store it in a
  // RefCell and get a reference to it
  _event_object :&'struct_lifetime RefCell<EventObject>
}

impl<'struct_lifetime> NameObserver<'struct_lifetime> {
  pub(super) fn new(
    event_object :&'struct_lifetime RefCell<EventObject>
  ) -> Self {
    Self {
      _event_object: event_object
    }
  }
}

impl IObserver<EventType>
  for NameObserver<'_> {
  fn notify(
    &mut self,
    data: &EventType
  ) {
    match data {
      EventType::Name(name) => {
        self._event_object.borrow_mut().name =
          name.clone();
      }
      _ => {
        // Do nothing for every other case
      }
    }
  }
}