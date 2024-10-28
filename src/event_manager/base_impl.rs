// This is a base implementation of our EventManager interface,
// which just stores a map of observers and notifies them

use std::collections::HashMap;
use uuid::Uuid;
use crate::event_manager::IEventManager;
use crate::event_manager::observer::IObserver;

pub struct BaseEventManagerImpl<'manager_lifetime, EventDataType> {
  _observers :HashMap<String, Box<dyn IObserver<EventDataType>+'manager_lifetime>>
}

impl<EventDataType> BaseEventManagerImpl<'_, EventDataType> {
  pub fn new() -> Self {
    Self {
      // Create a blank hash map that will store our observers
      _observers: HashMap::default()
    }
  }
}

impl<'manager_lifetime, EventDataType>
  IEventManager<'manager_lifetime, EventDataType>
  for BaseEventManagerImpl<'manager_lifetime, EventDataType> {
  fn add_observer(
    &mut self,
    observer: Box<dyn IObserver<EventDataType>+'manager_lifetime>
  ) -> String {
    // Allocate our observer key, use a UUID

    let observer_key =
      Uuid::new_v4().to_string();

    // Insert this observer into our hash map

    self._observers.insert(
      // Use a cloned version of our key to avoid
      // losing ownership of it
      observer_key.clone(),
      observer
    );

    // Return our observer key
    return observer_key;
  }

  fn remove_observer(
    &mut self,
    observer_id: &str
  ) {
    // Remove the observer if it exists

    self._observers.remove(
      observer_id
    );
  }

  fn notify(
    // Self has to be mutable as the observer within our hash map has to be mutable,
    // as the observer can be any class and has to be modifiable
    &mut self,
    event_data: &EventDataType
  ) {
    // Notify all our observers with the event data
    for (_observer_key, observer) in &mut self._observers {
      observer.notify(
        event_data
      );
    }
  }
}