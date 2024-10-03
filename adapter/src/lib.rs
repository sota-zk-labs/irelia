use ethers::prelude::U256;

mod annotated_proof;
mod annotation_parser;
mod builtin_info;
mod errors;
mod merkle_statement;
mod fri_merkle_statement;
mod oods_statement;
mod proof;


/// Default prime field for cairo. This prime will be used when modular operations are needed.
pub fn default_prime() -> U256 {
    U256::from(2).pow(U256::from(251))
        + U256::from(17) * U256::from(2).pow(U256::from(192))
        + U256::from(1)
}

#[cfg(test)]
mod tests {
    use super::*;

}
