use crate::mediator::IMediator;
use crate::tests::data::data_storage::DataStorage;

pub(super) struct TestDataMediator<'mediator_lifetime> {
  // We will mediate with our data storage
  _data_storage: &'mediator_lifetime mut DataStorage<u64>
}

impl<'mediator_lifetime> TestDataMediator<'mediator_lifetime> {
  pub(super) fn new(
    data_storage: &'mediator_lifetime mut DataStorage<u64>
  ) -> Self {
    Self {
      _data_storage: data_storage
    }
  }
}

impl IMediator<'_, u64> for TestDataMediator<'_> {
  fn execute(
    &mut self,
    data: u64
  ) {
    // Set our data to a doubled value

    self._data_storage.data =
      data * 2;
  }
}

