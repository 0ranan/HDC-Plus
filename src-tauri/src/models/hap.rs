use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstallTask {
    pub device_id: String,
    pub hap_path: String,
    pub status: InstallStatus,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InstallStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "installing")]
    Installing,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}
