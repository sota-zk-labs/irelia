use std::fs::File;
use std::io::BufReader;

use crate::config::AppConfig;
use crate::contracts_caller::memory_page_fact_registry::register_continuous_page_batch::register_continuous_page_batch;
use crate::contracts_caller::memory_page_fact_registry::types::memory_page_batch::MemoryPageEntries;

pub async fn register_memory(config: &AppConfig) -> anyhow::Result<bool> {
    let file_path =
        "src/data_samples/memory_page_fact_registry/register_continuos_page_batch.json".to_string();
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);
    let memory_page_entries: MemoryPageEntries = serde_json::from_reader(reader)?;

    Ok(register_continuous_page_batch(config, memory_page_entries).await?)
}
