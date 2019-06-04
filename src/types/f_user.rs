use rtdlib::types as td_types;

use crate::errors;
use crate::types::t_user::TGUser;
use crate::types::t_user_type_bot::TGUserTypeBot;

impl TGUser {
  pub fn id(&self) -> i32 { self.origin().id().clone().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn first_name(&self) -> &Option<String> { self.origin().first_name() }

  pub fn last_name(&self) -> &Option<String> { self.origin().last_name() }

  pub fn username(&self) -> &Option<String> { self.origin().username() }

  pub fn phone_number(&self) -> String { self.origin().phone_number().clone().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn status(&self) -> TGUserStatus { self.origin().status().clone().map(|v| TGUserStatus::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn profile_photo(&self) -> &Option<td_types::ProfilePhoto> { self.origin().profile_photo() }

  pub fn outgoing_link(&self) -> TGLinkState { self.origin().outgoing_link().clone().map(|v| TGLinkState::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn incoming_link(&self) -> TGLinkState { self.origin().incoming_link().clone().map(|v| TGLinkState::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn is_verified(&self) -> bool { self.origin().is_verified().clone().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn is_support(&self) -> bool { self.origin().is_support().clone().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn restriction_reason(&self) -> Option<String> { self.origin().restriction_reason().clone().filter(|v| !v.is_empty()) }

  pub fn have_access(&self) -> bool { self.origin().have_access().clone().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn type_(&self) -> TGUserType { self.origin().type_().clone().map(|v| TGUserType::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn language_code(&self) -> Option<String> { self.origin().language_code().clone().filter(|v| !v.is_empty()) }

  pub fn is_bot(&self) -> bool {
    match self.type_() {
      TGUserType::Bot(_) => true,
      _ => false
    }
  }

  pub fn is_deleted(&self) -> bool {
    match self.type_() {
      TGUserType::Deleted => true,
      _ => false
    }
  }

  pub fn is_regular(&self) -> bool {
    match self.type_() {
      TGUserType::Regular => true,
      _ => false
    }
  }
}

#[derive(Debug, Clone)]
pub enum TGUserType {
  Bot(TGUserTypeBot),
  Deleted,
  Regular,
  Unknown,
}

impl TGUserType {
  fn of(td: Box<td_types::UserType>) -> Self {
    match td_types::RTDUserTypeType::of(td.td_name()) {
      Some(td_types::RTDUserTypeType::UserTypeBot) => {
        TGUserType::Bot(TGUserTypeBot::from_json(td.to_json()).expect(errors::TELEGRAM_DATA_FAIL))
      }
      Some(td_types::RTDUserTypeType::UserTypeDeleted) => TGUserType::Deleted,
      Some(td_types::RTDUserTypeType::UserTypeRegular) => TGUserType::Regular,
      Some(td_types::RTDUserTypeType::UserTypeUnknown) => TGUserType::Unknown,
      None => panic!(errors::TELEGRAM_DATA_FAIL)
    }
  }
}


/// Represents the relationship between user A and user B. For incoming_link, user A is the current user; for outgoing_link, user B is the current user.
#[derive(Debug, Clone)]
pub enum TGLinkState {
  /// /// The phone number of user A has been saved to the contact list of user B.
  IsContact,
  /// The phone number of user A is known but that number has not been saved to the contact list of user B.
  KnowsPhoneNumber,
  /// The phone number of user A is not known to user B.
  None,
}

impl TGLinkState {
  fn of(td: Box<td_types::LinkState>) -> Self {
    match td_types::RTDLinkStateType::of(td.td_name()) {
      Some(td_types::RTDLinkStateType::LinkStateIsContact) => TGLinkState::IsContact,
      Some(td_types::RTDLinkStateType::LinkStateKnowsPhoneNumber) => TGLinkState::KnowsPhoneNumber,
      Some(td_types::RTDLinkStateType::LinkStateNone) => TGLinkState::None,
      None => panic!(errors::TELEGRAM_DATA_FAIL)
    }
  }
}

/// This class is an abstract base class. Describes the last time the user was online.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum TGUserStatus {
  /// The user status was never changed.
  Empty,
  /// The user is offline, but was online last month.
  LastMonth,
  /// The user is offline, but was online last week.
  LastWeek,
  /// The user is offline.
  ///
  /// Point in time (Unix timestamp) when the user's online status will expire.
  Offline(i32),
  /// The user is online.
  ///
  /// Point in time (Unix timestamp) when the user was last online.
  Online(i32),
  /// The user was online recently.
  Recently,
}

impl TGUserStatus {
  fn of(td: Box<td_types::UserStatus>) -> Self {
    match td_types::RTDUserStatusType::of(td.td_name()) {
      Some(td_types::RTDUserStatusType::UserStatusEmpty) => TGUserStatus::Empty,
      Some(td_types::RTDUserStatusType::UserStatusLastMonth) => TGUserStatus::LastMonth,
      Some(td_types::RTDUserStatusType::UserStatusLastWeek) => TGUserStatus::LastWeek,
      Some(td_types::RTDUserStatusType::UserStatusOffline) => {
        td_types::UserStatusOffline::from_json(td.to_json())
          .map(|v| TGUserStatus::Offline(v.was_online().clone().expect(errors::TELEGRAM_DATA_FAIL)))
          .expect(errors::TELEGRAM_DATA_FAIL)
      }
      Some(td_types::RTDUserStatusType::UserStatusOnline) => {
        td_types::UserStatusOnline::from_json(td.to_json())
          .map(|v| TGUserStatus::Online(v.expires().clone().expect(errors::TELEGRAM_DATA_FAIL)))
          .expect(errors::TELEGRAM_DATA_FAIL)
      }
      Some(td_types::RTDUserStatusType::UserStatusRecently) => TGUserStatus::Recently,
      None => panic!(errors::TELEGRAM_DATA_FAIL)
    }
  }
}



