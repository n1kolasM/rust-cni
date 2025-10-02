// Copyright (c) 2024 https://github.com/divinerapier/cni-rs

use std::io::stdout;

use serde::{Deserialize, Serialize};
use serde_json::to_string;

use crate::libcni::CNIError;

use super::APIResult;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Result {
    #[serde(rename = "cniVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cni_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<Vec<super::result100::Interface>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips: Option<Vec<super::result100::IPConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<crate::libcni::types::Route>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<crate::libcni::types::DNS>,
}

#[typetag::serde(name = "0.4.0")]
impl APIResult for Result {
    fn version(&self) -> String {
        if let Some(cni_version) = &self.cni_version {
            return cni_version.clone();
        }
        String::default()
    }

    fn get_as_version(&self, _version: String) -> super::ResultCNI<Box<dyn APIResult>> {
        Ok(Box::<Result>::default())
    }

    fn print(&self) -> super::ResultCNI<()> {
        self.print_to(Box::new(stdout()))
    }

    fn print_to(&self, mut w: Box<dyn std::io::Write>) -> super::ResultCNI<()> {
        let json_data = to_string(&self).unwrap();
        w.write(json_data.as_bytes())
            .map_err(|e| CNIError::Io(Box::new(e)))?;
        Ok(())
    }

    fn get_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn clone_box(&self) -> Box<dyn APIResult> {
        let cloned = Result {
            cni_version: self.cni_version.clone(),
            interfaces: self.interfaces.clone(),
            ips: self.ips.clone(),
            routes: self.routes.clone(),
            dns: self.dns.clone(),
        };
        Box::new(cloned)
    }
}
