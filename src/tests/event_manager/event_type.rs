use uuid::Uuid;

pub(super) enum EventType {
  Name(String),
  Payment(u64),
  Receipt(Uuid)
}