use serde::Deserialize;

use crate::contracts_caller::memory_page_fact_registry::types::register_continuous_memory_page::ContinuousMemoryPage;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MemoryPageEntries {
    pub memory_page_entries: Vec<ContinuousMemoryPage>,
}
