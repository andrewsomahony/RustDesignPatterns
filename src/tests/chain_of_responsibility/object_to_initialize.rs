pub(super) struct ObjectToInitialize {
  pub(super) name :String,
  pub(super) age :u64,
  pub(super) address :Vec<String>
}

impl ObjectToInitialize {
  pub(super)  fn new() -> Self {
    Self {
      name: String::default(),
      age: 0,
      address: Vec::new()
    }
  }
}