use ethers::prelude::U256;

mod annotation_parser;
mod builtin_info;
mod oods_statement;
mod sharp_prover;
mod stone_prover;

/// Default prime field for cairo. This prime will be used when modular operations are needed.
pub fn default_prime() -> U256 {
    U256::from(2).pow(U256::from(251))
        + U256::from(17) * U256::from(2).pow(U256::from(192))
        + U256::from(1)
}
