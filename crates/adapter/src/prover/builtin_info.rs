#[allow(dead_code)]
pub const OUTPUT_BUILTIN_BIT: usize = 0;
#[allow(dead_code)]
pub const PEDERSEN_BUILTIN_BIT: usize = 1;
#[allow(dead_code)]
pub const RANGE_CHECK_BUILTIN_BIT: usize = 2;
#[allow(dead_code)]
pub const ECDSA_BUILTIN_BIT: usize = 3;
#[allow(dead_code)]
pub const BITWISE_BUILTIN_BIT: usize = 4;
#[allow(dead_code)]
pub const EC_OP_BUILTIN_BIT: usize = 5;
#[allow(dead_code)]
pub const KECCAK_BUILTIN_BIT: usize = 6;
#[allow(dead_code)]
pub const POSEIDON_BUILTIN_BIT: usize = 7;
#[allow(dead_code)]
pub const N_BUILTINS: usize = 9;
#[allow(dead_code)]
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
