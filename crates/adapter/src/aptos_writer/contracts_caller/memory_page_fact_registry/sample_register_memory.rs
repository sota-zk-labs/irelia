use std::fs::File;
use std::io::BufReader;

use crate::aptos_writer::contracts_caller::memory_page_fact_registry::types::register_continuous_memory_page::ContinuousMemoryPage;
use crate::aptos_writer::contracts_caller::memory_page_fact_registry::types::register_continuous_page_batch::MemoryPageEntries;

pub fn sample_register_continuous_page_batch(index: u64) -> anyhow::Result<MemoryPageEntries> {
    let file_path = format!("./src/test_samples/test_samples/memory_page_fact_registry/register_continuous_page_batch_{}.json", index);
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);
    let memory_page_entries: MemoryPageEntries = serde_json::from_reader(reader)?;
    Ok(memory_page_entries)
}

pub fn sample_register_continuous_page(index: u64) -> anyhow::Result<ContinuousMemoryPage> {
    let file_path = format!(
        "./src/test_samples/test_samples/memory_page_fact_registry/register_memory_page_{}.json",
        index
    );
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);
    let continuous_memory_page: ContinuousMemoryPage = serde_json::from_reader(reader)?;
    Ok(continuous_memory_page)
}

pub fn sample_large_data_register_continuous_page_batch() -> anyhow::Result<MemoryPageEntries> {
    let byte = include_bytes!("../../test_samples/data_samples_friendly_layers/memory_page_fact_registry/large_data_register_continuous_page_batch.json");
    let memory_page_entries: MemoryPageEntries = serde_json::from_slice(byte)?;
    Ok(memory_page_entries)
}
