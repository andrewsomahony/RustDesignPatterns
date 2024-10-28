mod event_object;
mod name_observer;
mod event_type;
mod payment_observer;

use std::cell::RefCell;
use crate::event_manager::base_impl::BaseEventManagerImpl;
use crate::event_manager::IEventManager;
use crate::tests::event_manager::event_object::EventObject;
use crate::tests::event_manager::event_type::EventType;
use crate::tests::event_manager::name_observer::NameObserver;
use crate::tests::event_manager::payment_observer::PaymentObserver;

#[test]
fn test_event_manager() {
  // Store our shared event state in a RefCell

  let event_object =
    RefCell::new(
      EventObject::default()
    );

  // Create an event manager to use

  let mut event_manager :BaseEventManagerImpl<EventType> =
    BaseEventManagerImpl::new();

  let name_observer_id =
    event_manager.add_observer(
      Box::new(
        NameObserver::new(
          &event_object
        )
      )
    );

  // Add a second observer to make sure our mutable object is properly
  // handled with RefCell

  let payment_observer_id =
    event_manager.add_observer(
      Box::new(
        PaymentObserver::new(
          &event_object
        )
      )
    );

  // Notify our customer's name

  event_manager.notify(
    &EventType::Name(
      String::from("Andrew")
    )
  );

  // Make sure that our object was modified

  assert_eq!(
    event_object.borrow().name,
    String::from("Andrew")
  );

  // Notify about our payment

  event_manager.notify(
    &EventType::Payment(
      100
    )
  );

  // Make sure our payment has been set

  assert_eq!(
    event_object.borrow().payment,
    100
  );

  event_manager.remove_observer(
    &name_observer_id
  );

  event_manager.remove_observer(
    &payment_observer_id
  );
}