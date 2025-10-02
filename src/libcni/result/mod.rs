// Copyright (c) 2024 https://github.com/divinerapier/cni-rs
use json::JsonValue;

pub mod result031;
pub mod result040;
pub mod result100;
pub mod result110;

pub type ResultCNI<T> = std::result::Result<T, Box<super::error::CNIError>>;

#[typetag::serde(tag = "cniVersion", default_variant = "1.0.0")]
pub trait APIResult {
    fn version(&self) -> String;
    fn get_as_version(&self, version: String) -> ResultCNI<Box<dyn APIResult>>;
    fn print(&self) -> ResultCNI<()>;
    fn print_to(&self, w: Box<dyn std::io::Write>) -> ResultCNI<()>;
    fn get_json(&self) -> JsonValue;
    fn clone_box(&self) -> Box<dyn APIResult>;
}
