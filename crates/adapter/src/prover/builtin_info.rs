pub const OUTPUT_BUILTIN_BIT: usize = 0;
pub const PEDERSEN_BUILTIN_BIT: usize = 1;
pub const RANGE_CHECK_BUILTIN_BIT: usize = 2;
pub const ECDSA_BUILTIN_BIT: usize = 3;
pub const BITWISE_BUILTIN_BIT: usize = 4;
pub const EC_OP_BUILTIN_BIT: usize = 5;
pub const KECCAK_BUILTIN_BIT: usize = 6;
pub const POSEIDON_BUILTIN_BIT: usize = 7;

pub const N_BUILTINS: usize = 9;

pub const BOOTLOADER_LEN: usize = 728;
#[allow(dead_code)]
pub fn get_layout7_selected_builtins() -> usize {
    let selected_builtins = (1 << OUTPUT_BUILTIN_BIT)
        | (1 << PEDERSEN_BUILTIN_BIT)
        | (1 << RANGE_CHECK_BUILTIN_BIT)
        | (1 << BITWISE_BUILTIN_BIT)
        | (1 << POSEIDON_BUILTIN_BIT);
    selected_builtins as usize
}
