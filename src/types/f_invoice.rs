use rtdlib::types::LabeledPricePart;

use crate::types::TGInvoice;

impl TGInvoice {
  pub fn currency(&self) -> Option<String> { self.td_origin().currency() }

  pub fn price_parts(&self) -> Vec<LabeledPricePart> { self.td_origin().price_parts().map_or(vec![], |v| v) }

  pub fn is_test(&self) -> bool { self.td_origin().is_test().map_or(false, |v| v) }

  pub fn need_name(&self) -> bool { self.td_origin().need_name().map_or(false, |v| v) }

  pub fn need_phone_number(&self) -> bool { self.td_origin().need_phone_number().map_or(false, |v| v) }

  pub fn need_email_address(&self) -> bool { self.td_origin().need_email_address().map_or(false, |v| v) }

  pub fn need_shipping_address(&self) -> bool { self.td_origin().need_shipping_address().map_or(false, |v| v) }

  pub fn send_phone_number_to_provider(&self) -> bool { self.td_origin().send_phone_number_to_provider().map_or(false, |v| v) }

  pub fn send_email_address_to_provider(&self) -> bool { self.td_origin().send_email_address_to_provider().map_or(false, |v| v) }

  pub fn is_flexible(&self) -> bool { self.td_origin().is_flexible().map_or(false, |v| v) }
}

