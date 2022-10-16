use serde::Serialize;

#[inline]
pub fn to_json_string(value: &(impl ?Sized + Serialize)) -> String {
    serde_json::to_string(value).expect("json serialization never fail; qed")
}

#[inline]
pub fn to_json_string_pretty(value: &(impl ?Sized + Serialize)) -> String {
    serde_json::to_string_pretty(value).expect("json serialization never fail; qed")
}

#[inline]
pub fn to_json_value(value: &(impl ?Sized + Serialize)) -> serde_json::Value {
    serde_json::to_value(value).expect("json serialization never fail; qed")
}

#[inline]
pub fn to_json_vec(value: &(impl ?Sized + Serialize)) -> Vec<u8> {
    serde_json::to_vec(value).expect("json serialization never fail; qed")
}
