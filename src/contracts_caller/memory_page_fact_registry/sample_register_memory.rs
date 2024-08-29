use std::fs::File;
use std::io::BufReader;

use crate::contracts_caller::memory_page_fact_registry::types::register_continuous_memorypage::ContinuousMemorypage;
use crate::contracts_caller::memory_page_fact_registry::types::register_continuous_page_batch::MemoryPageEntries;

pub fn sample_register_continuous_page_batch() -> anyhow::Result<MemoryPageEntries> {
    let file_path =
        "src/data_samples/memory_page_fact_registry/register_continuous_page_batch.json"
            .to_string();
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);
    let memory_page_entries: MemoryPageEntries = serde_json::from_reader(reader)?;
    Ok(memory_page_entries)
}

pub fn sample_register_continuous_page() -> anyhow::Result<ContinuousMemorypage> {
    let file_path =
        "src/data_samples/memory_page_fact_registry/register_memory_page.json".to_string();
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);
    let continuous_memmory_page: ContinuousMemorypage = serde_json::from_reader(reader)?;
    Ok(continuous_memmory_page)
}

pub fn sample_large_data_register_continuous_page_batch() -> anyhow::Result<MemoryPageEntries> {
    let file_path =
        "src/data_samples/memory_page_fact_registry/large_data_register_continuous_page_batch.json"
            .to_string();
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);
    let memory_page_entries: MemoryPageEntries = serde_json::from_reader(reader)?;
    Ok(memory_page_entries)
}
