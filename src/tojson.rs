//! Implements `to_json()`.
use rustc_serialize::json::{ToJson, Json};

use super::display::{Prefix,Postfix};
impl ToJson for Postfix{
    fn to_json(&self) -> Json{
        Json::String(self.to_string())
    }
}
impl ToJson for Prefix{
    fn to_json(&self) -> Json{
        Json::String(self.to_string())
    }
}

