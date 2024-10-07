use crate::contracts_caller::memory_page_fact_registry::types::register_continuous_memory_page::ContinuousMemoryPage;
use crate::contracts_caller::memory_page_fact_registry::types::register_continuous_page_batch::MemoryPageEntries;

pub fn extract_register_continuous_page_batch(
    memory_pages: &[String],
) -> anyhow::Result<Vec<MemoryPageEntries>> {
    let mut res: Vec<MemoryPageEntries> = vec![];
    for memory_page in memory_pages {
        let memory_page_entries: MemoryPageEntries = serde_json::from_str(memory_page)?;
        res.push(memory_page_entries)
    }
    Ok(res)
}

pub fn extract_register_continuous_page(
    memory_pages: &[String],
) -> anyhow::Result<Vec<ContinuousMemoryPage>> {
    let mut res: Vec<ContinuousMemoryPage> = vec![];
    for memory_page in memory_pages {
        let continuous_memory_page: ContinuousMemoryPage = serde_json::from_str(memory_page)?;
        res.push(continuous_memory_page)
    }
    Ok(res)
}

pub fn extract_large_data_register_continuous_page_batch(
    memory_pages: &[String],
) -> anyhow::Result<Vec<MemoryPageEntries>> {
    let mut res: Vec<MemoryPageEntries> = vec![];
    for memory_page in memory_pages {
        let memory_page_entries: MemoryPageEntries = serde_json::from_str(memory_page)?;
        res.push(memory_page_entries)
    }
    Ok(res)
}
