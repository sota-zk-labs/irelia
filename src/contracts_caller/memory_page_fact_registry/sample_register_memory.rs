use crate::contracts_caller::memory_page_fact_registry::types::register_continuous_memory_page::ContinuousMemoryPage;
use crate::contracts_caller::memory_page_fact_registry::types::register_continuous_page_batch::MemoryPageEntries;

pub fn sample_register_continuous_page_batch() -> anyhow::Result<MemoryPageEntries> {
    let byte = include_bytes!(
        "../../data_samples/memory_page_fact_registry/register_continuous_page_batch.json"
    );
    let memory_page_entries: MemoryPageEntries = serde_json::from_slice(byte)?;
    Ok(memory_page_entries)
}

pub fn sample_register_continuous_page() -> anyhow::Result<ContinuousMemoryPage> {
    let byte =
        include_bytes!("../../data_samples/memory_page_fact_registry/register_memory_page.json");
    let continuous_memory_page: ContinuousMemoryPage = serde_json::from_slice(byte)?;
    Ok(continuous_memory_page)
}

pub fn sample_large_data_register_continuous_page_batch() -> anyhow::Result<MemoryPageEntries> {
    let byte = include_bytes!("../../data_samples/memory_page_fact_registry/large_data_register_continuous_page_batch.json");
    let memory_page_entries: MemoryPageEntries = serde_json::from_slice(byte)?;
    Ok(memory_page_entries)
}
