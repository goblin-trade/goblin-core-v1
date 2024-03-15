pub trait RestingOrder {
  fn size(&self) -> u32;
  fn last_valid_slot(&self) -> Option<u32>;
  fn last_valid_unix_timestamp_in_seconds(&self) -> Option<u32>;
  fn is_expired(&self, current_slot: u32, current_unix_timestamp_in_seconds: u32) -> bool;
}
