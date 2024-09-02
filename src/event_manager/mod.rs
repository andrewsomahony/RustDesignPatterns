pub mod observer;
pub mod base_impl;

use crate::event_manager::observer::IObserver;

pub trait IEventManager<'trait_lifetime, EventDataType> {
  // Our observer IDs are all strings, so we can use
  // GUID's if an implementation wants to
  fn add_observer(
    &mut self,
    // The event manager takes ownership of the observer
    // object
    observer :Box<dyn IObserver<'trait_lifetime, EventDataType>+'trait_lifetime>
  ) -> String;

  // Removes an observer with the given ID

  fn remove_observer(
    &mut self,
    observer_id :String
  );

  fn notify(
    &mut self,
    event_data :EventDataType
  );
}