pub mod observer;
pub mod base_impl;

use crate::event_manager::observer::IObserver;

// We have a lifetime here because although we own our observers
// via them being passed as a Box with a value, we need to make sure
// that every reference inside the observers lives at least as long as
// the event manager itself.  If we don't specify an explicit lifetime for
// the Boxed observer, it will default to "'static", which isn't what we
// want in most occasions for our member variable lifetimes

pub trait IEventManager<'trait_lifetime, EventDataType> {
  // Our observer IDs are all strings, so we can use
  // GUID's if an implementation wants to
  fn add_observer(
    &mut self,
    // The event manager takes ownership of the observer
    // object
    observer :Box<dyn IObserver<EventDataType>+'trait_lifetime>
  ) -> String;

  // Removes an observer with the given ID

  fn remove_observer(
    &mut self,
    observer_id :&str
  );

  fn notify(
    &mut self,
    // We don't need to own the event data, just borrow it,
    // as it's immutable
    event_data :&EventDataType
  );
}