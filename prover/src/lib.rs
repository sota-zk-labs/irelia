use std::path::PathBuf;
use stone_cli::args::LayoutName;

// pub fn generate_proof(
//     cairo_programs: Option<Vec<PathBuf>>,
//     cairo_pies: Option<Vec<PathBuf>>,
//     layout: LayoutName)
// -> Result<(), String> {
//     if layout != LayoutName::starknet {
//         Err("Layout is not supported");
//     }
//     let arg = stone_cli::args::ProveBootloaderArgs {
//         cairo_programs: cairo_programs,
//         cairo_pies,
//         layout: layout,
//         prover_config_file: None,
//         parameter_file: None,
//         output: Default::default(),
//         fact_topologies_output: Default::default(),
//         parameter_config: Default::default(),
//         prover_config: Default::default(),
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;


}
