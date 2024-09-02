// This is the object that is maintained by our observers.  The observer
// will get a piece of data indicating which field to update, and will update
// that field

use uuid::Uuid;

#[derive(Default)]
pub(super) struct EventObject {
  pub(super) name :String,
  pub(super) payment :u64,
  pub(super) receipt_hash :Uuid
}

// No need to implement as we have just derived from
// Default, which will create default values for our
// struct elements