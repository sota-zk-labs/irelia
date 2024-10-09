pub struct SharpProof {
    pub merkle_proofs: Vec<String>,
    pub fri_proofs: Vec<String>,
    pub memory_pages: Vec<String>,
    pub main_proof: String,
    pub layout: usize,
}
