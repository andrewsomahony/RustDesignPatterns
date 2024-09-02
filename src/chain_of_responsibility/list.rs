use std::collections::LinkedList;
use std::error::Error;
use crate::chain_of_responsibility::IChainOfResponsibilityHandler;

// The chain of responsibility list is a Chain of Responsibility Handler,
// but its "handle" method simply runs through its linked list of handlers

pub struct ChainOfResponsibilityList<'struct_lifetime, ObjectType> {
  // The trait has to live at least as long as the struct
  _list :LinkedList<&'struct_lifetime dyn IChainOfResponsibilityHandler<'struct_lifetime, ObjectType>>,
}

impl<'struct_lifetime, ObjectType> ChainOfResponsibilityList<'struct_lifetime, ObjectType> {
  // New method to create the list

  pub fn new(
  ) -> Self {
    return Self {
      _list: LinkedList::default()
    }
  }

  // Adds a new handler to our list

  pub fn add_handler(
    &mut self,
    handler :&'struct_lifetime dyn IChainOfResponsibilityHandler<'struct_lifetime, ObjectType>
  ) {
    // Add the handler to our list

    self._list.push_back(
      handler
    )
  }

  // Adds a list to our list, clearing the old list
  // Takes ownership of the old list as well

  pub fn add_list(
    &mut self,
    mut list: ChainOfResponsibilityList<'struct_lifetime, ObjectType>
  ) {
    self._list.append(
      &mut list._list
    );
  }
}

impl<'struct_lifetime, ObjectType> IChainOfResponsibilityHandler<'struct_lifetime, ObjectType>
  for ChainOfResponsibilityList<'struct_lifetime, ObjectType> {
  fn handle(
    &self,
    object: &mut ObjectType
  ) -> Result<(), Box<dyn Error>> {
    // Simply iterate through our list and execute each handler on
    // the input object
    // If one handler fails, an Exception will be thrown

    // We borrow self._list, which Rust will automatically infer
    // to call iter on the list instead of into_iter, which makes sure
    // we aren't transferring ownership

    for handler in &self._list {
      // Box supports deref coercion, so we don't have to explicitly
      // deref our handler object
      handler.handle(
        object
      )?;
    }

    return Ok(());
  }
}