#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;
extern crate slog_term;

use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use rtdlib::types as td_types;

use telegram_client::api::Api;
use telegram_client::client::Client;
use telegram_client::types::*;

use crate::proxy::TProxy;
use std::time::Duration;
use std::sync::mpsc::TryRecvError;

mod exmlog;
mod proxy;
mod thelp;
mod tgfn;

fn main() {
  let log_file = toolkit::path::root_dir().join("tdlib.log");
  if log_file.exists() {
    std::fs::remove_file(&log_file);
  }
  File::create(&log_file).unwrap();

  Client::set_log_verbosity_level(1);
//  Client::set_log_file_path(Some(&toolkit::path::canonicalize_path(log_file).unwrap()[..]));

  let api = Api::default();
  let mut client = Client::new(api.clone());

  let tproxy = TProxy::new(&api);
  tproxy.add();


  let listener = client.listener();

  let have_authorization: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

//  listener.on_receive(|(api, object)| {
//    println!("{:?}", object);
//  });

  listener.on_option(|(api, option)| {
    let value = option.value();
    if value.is_empty() { debug!(exmlog::examples(), "Receive an option {} but it's empty", option.name()) }
    if value.is_string() { debug!(exmlog::examples(), "Receive an option {}: String => {}", option.name(), value.as_string().map_or("None".to_string(), |v| v)) }
    if value.is_integer() { debug!(exmlog::examples(), "Receive an option {}: i32 => {}", option.name(), value.as_integer().map_or(-1, |v| v)) }
    if value.is_bool() { debug!(exmlog::examples(), "Receive an option {}: bool => {}", option.name(), value.as_bool().map_or(false, |v| v)) }

    option.on_name("version", |value| {
      value.as_string().map(|v| { debug!(exmlog::examples(), "VERSION IS {}", v); });
    });
  });

  listener.on_authorization_state(move |(api, state)| {
    state.on_wait_tdlibparameters(|| {
      let paras = td_types::SetTdlibParameters::builder()
        .parameters(td_types::TdlibParameters::builder()
          .database_directory("tdlib")
          .use_message_database(true)
          .use_secret_chats(true)
          .api_id(toolkit::number::as_i32(env!("API_ID")).unwrap())
          .api_hash(env!("API_HASH"))
          .system_language_code("en")
          .device_model("Desktop")
          .system_version("Unknown")
          .application_version(env!("CARGO_PKG_VERSION"))
          .enable_storage_optimizer(true)
          .build())
        .build();
      api.send(&paras);
      debug!(exmlog::examples(), "Set tdlib parameters");
    });
    state.on_wait_encryption_key(|enck| {
      api.send(td_types::CheckDatabaseEncryptionKey::builder().build());
      debug!(exmlog::examples(), "Set encryption key");
    });
    state.on_wait_phone_number(|| {
      thelp::tip("Please type your telegram phone number:");
      tgfn::type_phone_number(api);
    });
    state.on_wait_password(|aswp| {
      api.send(td_types::CheckAuthenticationPassword::builder()
        .password(thelp::typed_with_message("Please type your telegram password:"))
        .build());
      debug!(exmlog::examples(), "Set password *****");
    });
    state.on_wait_code(|awc| {
      if awc.is_registered().clone().map_or(false, |v| v) {
        thelp::tip("Please type authentication code:");
        tgfn::type_authentication_code(api);
      } else {
        thelp::tip("Welcome to use telegram");
        thelp::tip("Your phone number is not registed to telegramm, please type your name. and authentication code.");
        tgfn::type_authentication_code_register(api);
      }
    });

    state.on_ready(|| {
      let mut have_authorization = have_authorization.lock().unwrap();
      *have_authorization = true;
      debug!(exmlog::examples(), "Authorization ready");
    });
    state.on_logging_out(|| {
      let mut have_authorization = have_authorization.lock().unwrap();
      *have_authorization = false;
      debug!(exmlog::examples(), "Logging out");
    });
    state.on_closing(|| {
      let mut have_authorization = have_authorization.lock().unwrap();
      *have_authorization = false;
      debug!(exmlog::examples(), "Closing");
    });
    state.on_closed(|| {
      debug!(exmlog::examples(), "Closed");
    });
  });

  listener.on_connection_state(|(api, update)| {
    update.on_state(|state| {
      match state {
        TGConnectionState::WaitingForNetwork => {
          debug!(exmlog::examples(), "waiting for network")
        }
        TGConnectionState::ConnectingToProxy => {
          debug!(exmlog::examples(), "connection to proxy")
        }
        TGConnectionState::Connecting => {
          debug!(exmlog::examples(), "connecting")
        }
        TGConnectionState::Updating => {
          debug!(exmlog::examples(), "updating...")
        }
        TGConnectionState::Ready => {
          debug!(exmlog::examples(), "connection ready")
        }
      }
    });
  });

  listener.on_error(|(api, error)| {
    let code = error.code().clone().map_or(-1, |v| v);
    let message = error.message().clone().map_or("None".to_string(), |v| v);
    error!(exmlog::examples(), "ERROR [{}] {}", code, message);
    match code {
      8 => {
        thelp::tip(&message);
        thelp::tip("Please type telegram phone number");
        tgfn::type_phone_number(api);
      }
      400 => {
        match &message[..] {
          "PHONE_NUMBER_INVALID" => {
            thelp::tip("Phone number invalid, please type a right phone number for telegram");
            tgfn::type_phone_number(api);
          }
          "PHONE_CODE_INVALID" | "PHONE_CODE_EMPTY" => {
            thelp::tip("Phone code invalid, please type an authentication code");
            tgfn::type_authentication_code(api);
          }
          _ => {}
        }
      }
      429 => thelp::wait_too_many_requests(api, &message),
      _ => thelp::unknown(code, &message)
    }
  });

  listener.on_ok(|api| {
    debug!(exmlog::examples(), "OK");
  });

  listener.on_proxy(|(api, pxy)| {
    debug!(exmlog::examples(), "Proxy info => {:?}", pxy);
  });

  listener.on_update_user(|(api, user)| {
    debug!(exmlog::examples(), "Update user => {:?}", user);
  });


  client.daemon("telegram-rs");
}

