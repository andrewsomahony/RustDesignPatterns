pub trait IObserver<'trait_lifetime, ObserverDataType> {
  fn notify(
    &mut self,
    // The data is immutable, but we are just borrowing it when
    // we are notified
    data :&ObserverDataType
  );
}