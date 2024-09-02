mod event_object;
mod test_observer;
mod event_type;

use crate::event_manager::base_impl::BaseEventManagerImpl;
use crate::event_manager::IEventManager;
use crate::tests::event_manager::event_object::EventObject;
use crate::tests::event_manager::event_type::EventType;
use crate::tests::event_manager::test_observer::TestObserver;

#[test]
fn test_event_manager() {
  let mut event_object :EventObject =
    EventObject::default();

  let mut event_observer :TestObserver =
    TestObserver::new(
      &mut event_object
    );

  // Create an event manager to use

  let mut event_manager :BaseEventManagerImpl<EventType> =
    BaseEventManagerImpl::new();

  let observer_id =
    event_manager.add_observer(
      &mut event_observer
    );

  event_manager.notify(
    EventType::Name(
      String::from("Andrew")
    )
  );

  event_manager.remove_observer(
    observer_id
  );

  assert_eq!(
    event_object.name,
    String::from("Andrew")
  );
}