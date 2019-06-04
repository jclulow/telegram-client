use rtdlib::types as td_types;

use crate::types::TGAuthorizationState;

macro_rules! fn_noarg {
  ($class:ident) => (
    fn fn_noarg<F: FnOnce()>(state: &Option<Box<td_types::AuthorizationState>>, fnc: F) {
      state.clone().filter(|ae| td_types::RTDType::of(ae.td_name()) == Some(td_types::RTDType::$class))
        .map(|ae| { fnc(); });
    }
  );
}

macro_rules! fn_td {
  ($class:ident) => (
    fn fn_td<F: FnOnce(&td_types::$class)>(state: &Option<Box<td_types::AuthorizationState>>, fnc: F) {
      state.clone()
        .filter(|ae| td_types::RTDType::of(ae.td_name()) == Some(td_types::RTDType::$class))
        .map(|ae| td_types::$class::from_json(ae.to_json()))
        .filter(|we| we.is_some())
        .map(|we| we.clone().map(|we| fnc(&we)));
    }
  );
}

impl TGAuthorizationState {
  fn authorization_state(&self) -> &Option<Box<td_types::AuthorizationState>> {
    self.origin().authorization_state()
  }

  pub fn on_wait_tdlibparameters<F: FnOnce()>(&self, fnc: F) -> &Self {
    fn_noarg!(AuthorizationStateWaitTdlibParameters);
    fn_noarg(self.authorization_state(), fnc);
    self
  }

  pub fn on_wait_encryption_key<F: FnOnce(&td_types::AuthorizationStateWaitEncryptionKey)>(&self, fnc: F) -> &Self {
    fn_td!(AuthorizationStateWaitEncryptionKey);
    fn_td(self.authorization_state(), fnc);
    self
  }

  pub fn on_wait_phone_number<F: FnOnce()>(&self, fnc: F) -> &Self {
    fn_noarg!(AuthorizationStateWaitPhoneNumber);
    fn_noarg(self.authorization_state(), fnc);
    self
  }

  pub fn on_wait_password<F: FnOnce(&td_types::AuthorizationStateWaitPassword)>(&self, fnc: F) -> &Self {
    fn_td!(AuthorizationStateWaitPassword);
    fn_td(self.authorization_state(), fnc);
    self
  }

  pub fn on_state_ready<F: FnOnce()>(&self, fnc: F) -> &Self {
    fn_noarg!(AuthorizationStateReady);
    fn_noarg(self.authorization_state(), fnc);
    self
  }

  pub fn on_state_logging_out<F: FnOnce()>(&self, fnc: F) -> &Self {
    fn_noarg!(AuthorizationStateLoggingOut);
    fn_noarg(self.authorization_state(), fnc);
    self
  }

  pub fn on_state_closing<F: FnOnce()>(&self, fnc: F) -> &Self {
    fn_noarg!(AuthorizationStateClosing);
    fn_noarg(self.authorization_state(), fnc);
    self
  }

  pub fn on_state_closed<F: FnOnce()>(&self, fnc: F) -> &Self {
    fn_noarg!(AuthorizationStateClosed);
    fn_noarg(self.authorization_state(), fnc);
    self
  }
}
