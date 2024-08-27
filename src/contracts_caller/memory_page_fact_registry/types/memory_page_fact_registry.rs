use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisterMemoryPage {
    pub start_addr: String,
    pub values: Vec<String>,
    pub z: String,
    pub alpha: String,
    pub prime: String,
}
