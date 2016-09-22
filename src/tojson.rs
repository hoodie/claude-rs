//! Implements `to_json()`.
use rustc_serialize::json::{ToJson, Json};

use super::{Prefix,Postfix};
impl<'a> ToJson for Postfix<'a>{
    fn to_json(&self) -> Json{
        Json::String(self.to_string())
    }
}
impl<'a> ToJson for Prefix<'a>{
    fn to_json(&self) -> Json{
        Json::String(self.to_string())
    }
}

