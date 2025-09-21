use super::super::common::structs::*;
use serde::{Deserialize, Serialize};

pub type LicenseKeyInstanceResponse = LicenseKeyInstance;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListLicenseKeyInstancesResponse {
    pub items: Vec<LicenseKeyInstanceResponse>,
}
