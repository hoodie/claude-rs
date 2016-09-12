//! Implements `to_json()`.
use rustc_serialize::json::{ToJson, Json};

use super::Currency;
impl ToJson for Currency{
    fn to_json(&self) -> Json{
        Json::String(self.to_string())
    }
}

