use crate::contracts_caller::memory_page_fact_registry::types::memory_page_fact_registry::RegisterMemoryPage;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MemoryPageEntries {
    pub memory_page_entries: Vec<RegisterMemoryPage>,
}
