use std::collections::{HashMap, HashSet};

use ethers::abi::Token;
use ethers::utils::keccak256;
use ethers::{types::U256, utils::hex};
use num_bigint::BigUint;
use num_traits::{Num, One};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::annotated_proof::AnnotatedProof;
use crate::errors::ParseError;
use crate::fri_merkle_statement::FRIMerkleStatement;
use crate::merkle_statement::MerkleStatement;
use crate::oods_statement::MainProof;

/// Adapted from https://github.com/zksecurity/stark-evm-adapter/blob/main/src/annotation_parser.rs
#[derive(Serialize, Deserialize, Debug, Clone)]
struct MerkleLine {
    pub name: String,
    pub node: U256,
    pub digest: String,
    pub annotation: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FriLine {
    pub name: String,
    pub row: usize,
    pub col: usize,
    pub element: String,
    pub annotation: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FriXInvLine {
    pub name: String,
    pub index: usize,
    pub inv: String,
    pub annotation: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CommitmentLine {
    pub name: String,
    pub digest: String,
    pub annotation: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EvalPointLine {
    pub name: String,
    pub point: String,
    pub annotation: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FriExtras {
    pub values: Vec<FriLine>,
    pub inverses: Vec<FriXInvLine>,
}

type MerkleExtrasDict = HashMap<String, Vec<MerkleLine>>;

struct FriMerklesOriginal {
    pub merkle_originals: MerkleExtrasDict,
    pub merkle_commitments: HashMap<String, CommitmentLine>,
    pub fri_originals: HashMap<String, Vec<FriLine>>,
    pub eval_points: Vec<EvalPointLine>,
    pub fri_names: Vec<String>,
    pub original_proof: Vec<u8>,
    pub merkle_patches: HashSet<String>,
}

#[derive(Serialize, Deserialize, Debug)]
/// [SplitProofs] maps the split proof json file which contains the main proof and the merkle statements
pub struct SplitProofs {
    pub main_proof: MainProof,
    pub merkle_statements: HashMap<String, MerkleStatement>,
    pub fri_merkle_statements: Vec<FRIMerkleStatement>,
}

// Parses hex strings and pads with zeros to make it 64 characters long
fn extract_hex(line: &str) -> Result<String, ParseError> {
    let re = Regex::new(r"\(0x([0-9a-f]+)\)")?;
    Ok(re
        .captures(line)
        .and_then(|cap| cap.get(1))
        .map_or_else(String::new, |m| format!("{:0>64}", m.as_str())))
}

/// Example:
/// /cpu air/STARK/FRI/Decommitment/Layer 0/Virtual Oracle/Trace 0: For node 18762888: Hash(0x0000000000000000000000006671d703e83592b3b9d60eb1f91e8e0b8561f3d1)
fn is_merkle_line(line: &str) -> bool {
    line.contains("Decommitment") && line.contains("node") && line.contains("Hash")
}

/// Parses a proof annotation line which is part of a Merkle decommitment, and returns the name
/// of the Merkle, the node provided, and the hash digest.
fn parse_merkle_line(line: &str) -> Result<MerkleLine, ParseError> {
    let name = line
        .split('/')
        .last()
        .ok_or(ParseError::InvalidLineFormat)?
        .split(':')
        .next()
        .ok_or(ParseError::InvalidLineFormat)?
        .to_string();

    let node_str = line
        .split("node ")
        .nth(1)
        .ok_or(ParseError::InvalidLineFormat)?
        .split(':')
        .next()
        .ok_or(ParseError::InvalidLineFormat)?;

    let node: U256 = U256::from_dec_str(node_str)?;

    let digest = extract_hex(line)?;

    Ok(MerkleLine {
        name,
        node,
        digest,
        annotation: line.to_string(),
    })
}

fn is_merkle_data_line(line: &str) -> bool {
    line.contains("Decommitment") && line.contains("element #") && line.contains("Data")
}

/// Parses a proof annotation line which is part of a Merkle decommitment as a package completion
/// line, and returns the name of the Merkle, the node provided, and the data element.
fn parse_merkle_data_line(line: &str) -> Result<MerkleLine, ParseError> {
    let name = line
        .split('/')
        .last()
        .ok_or(ParseError::InvalidLineFormat)?
        .split(':')
        .next()
        .ok_or(ParseError::InvalidLineFormat)?
        .to_string();

    let node_str = line
        .split("element #")
        .nth(1)
        .ok_or(ParseError::InvalidLineFormat)?
        .split(':')
        .next()
        .ok_or(ParseError::InvalidLineFormat)?;

    let node: U256 = U256::from_dec_str(node_str)?;

    let digest = extract_hex(line)?;

    Ok(MerkleLine {
        name,
        node,
        digest,
        annotation: line.to_string(),
    })
}

/// Example:
/// /cpu air/STARK/FRI/Decommitment/Layer 4: Row 242, Column 3: Field Element(0x32bb09aed5ade9cadd9c0e76f261422cb1ef1f483f28cfe06b81d416defde66)
fn is_fri_line(line: &str) -> bool {
    line.contains("Decommitment")
        && line.contains("Row")
        && line.contains("Field Element")
        && !line.contains("Virtual Oracle")
}

/// Parses a proof annotation line which is part of a FRI decommitment, and returns the name
/// of the FRI, the cell provided, and the field element.
fn parse_fri_line(line: &str) -> Result<FriLine, ParseError> {
    let parts: Vec<&str> = line.split('/').collect();
    let name = parts
        .last()
        .ok_or(ParseError::InvalidLineFormat)?
        .split(':')
        .next()
        .ok_or(ParseError::InvalidLineFormat)?
        .to_string();

    let row_col_part = line
        .split(':')
        .nth_back(1)
        .ok_or(ParseError::InvalidLineFormat)?;
    let row_col: Vec<&str> = row_col_part.split(',').collect();
    if row_col.len() != 2 {
        return Err(ParseError::InvalidLineFormat);
    }
    let row = row_col[0]
        .split_whitespace()
        .nth(1)
        .ok_or(ParseError::InvalidLineFormat)?
        .parse::<usize>()?;
    let col = row_col[1]
        .split_whitespace()
        .nth(1)
        .ok_or(ParseError::InvalidLineFormat)?
        .parse::<usize>()?;

    let element = extract_hex(line)?;

    Ok(FriLine {
        name,
        row,
        col,
        element,
        annotation: line.to_string(),
    })
}

/// Example:
/// /cpu air/STARK/FRI/Decommitment/Last Layer: xInv for index 2030: Field Element(0x6446597268afd0e19853257f3caf5f1d4bd896ae7edeba0047d8c09cf399b7b)
fn is_fri_xinv_line(line: &str) -> bool {
    line.contains("Decommitment") && line.contains("xInv") && line.contains("Field Element")
}

/// Parses a proof extra annotation line which contains the xInv for a FRI query, and returns the
/// name of the FRI, the index provided, and the xInv.
fn parse_fri_xinv_line(line: &str) -> Result<FriXInvLine, ParseError> {
    let parts: Vec<&str> = line.split('/').collect();
    let name = parts
        .last()
        .ok_or(ParseError::InvalidLineFormat)?
        .split(':')
        .next()
        .ok_or(ParseError::InvalidLineFormat)?
        .to_string();

    let index_str = line
        .split("index ")
        .nth(1)
        .ok_or(ParseError::InvalidLineFormat)?
        .split(':')
        .next()
        .ok_or(ParseError::InvalidLineFormat)?;

    let index = index_str
        .parse::<usize>()
        .map_err(|_| ParseError::InvalidLineFormat)?;

    let inv = extract_hex(line)?;

    Ok(FriXInvLine {
        name,
        index,
        inv,
        annotation: line.to_string(),
    })
}

/// Example
/// P->V[23680:23712]: /cpu air/STARK/FRI/Commitment/Layer 4: Commitment: Hash(0x000000000000000000000000ab09e126b366725542268572e3303dcaccf142a5)
fn is_commitment_line(line: &str) -> bool {
    line.contains("Commitment") && line.contains("Hash")
}

/// Parses a proof annotation line of a Merkle commitment, and returns the name
/// of the Merkle and the hash digest.
/// For "Commit on Trace" lines, the name (index) of the trace is not found inside the line, but
/// instead is tracked by a global counter.
fn parse_commitment_line(
    line: &str,
    trace_commitment_counter: &mut usize,
) -> Result<(CommitmentLine, usize), ParseError> {
    let parts: Vec<&str> = line.split(':').collect();
    let path_parts: Vec<&str> = parts
        .get(2)
        .ok_or(ParseError::InvalidLineFormat)?
        .trim()
        .split('/')
        .collect();
    let name;
    let new_trace_commitment_counter;

    if path_parts.last() == Some(&"Commit on Trace") {
        name = format!("Trace {}", trace_commitment_counter);
        *trace_commitment_counter += 1;
        new_trace_commitment_counter = *trace_commitment_counter;
    } else if path_parts.get(path_parts.len().saturating_sub(2)) == Some(&"Commitment") {
        name = path_parts
            .last()
            .map(|s| s.to_string())
            .ok_or(ParseError::InvalidLineFormat)?;
        new_trace_commitment_counter = *trace_commitment_counter;
    } else {
        return Err(ParseError::InvalidLineFormat);
    }

    let digest = extract_hex(line)?;
    Ok((
        CommitmentLine {
            name,
            digest,
            annotation: line.to_string(),
        },
        new_trace_commitment_counter,
    ))
}

/// Example:
/// V->P: /cpu air/STARK/FRI/Commitment/Layer 4: Evaluation point: Field Element(0x6afcea9769e097e3d5c3cb8ee22bcc51ed7e8cf8cfa617ee915a6af68736fee)
fn is_eval_point_line(line: &str) -> bool {
    line.contains("Evaluation point") && line.contains("Layer")
}

/// Parses a proof annotation line of an evaluation point submission, returning the layer number
/// and point.
fn parse_eval_point_line(line: &str) -> Result<EvalPointLine, ParseError> {
    let parts: Vec<&str> = line.split('/').collect();
    let name = parts
        .last()
        .ok_or(ParseError::InvalidLineFormat)?
        .split(':')
        .next()
        .ok_or(ParseError::InvalidLineFormat)?
        .to_string();

    let point = extract_hex(line)?;

    Ok(EvalPointLine {
        name,
        point,
        annotation: line.to_string(),
    })
}

/// Parses a proof annotation line, returning the start and end indices of the proof segment
/// which the line annotates, or (0,0) if the annotation is for verifier to prover "interaction".
fn line_to_indices(line: &str) -> Result<(usize, usize), ParseError> {
    if !line.starts_with("P->V[") {
        Ok((0, 0))
    } else {
        let indices_part = &line[5..line.find(']').ok_or(ParseError::InvalidLineFormat)?];
        let indices: Vec<&str> = indices_part.split(':').collect();
        if indices.len() != 2 {
            Err(ParseError::InvalidLineFormat)
        } else {
            let start = indices[0].parse::<usize>()?;
            let end = indices[1].parse::<usize>()?;
            Ok((start, end))
        }
    }
}

/// For a single Merkle decommitment, processes the information from the non-split proof
/// and extra data (merkle queue values) prepared by a verifier, and arranges it to be
/// used as input to the Merkle Fact Registry.
fn gen_merkle_statement_call(
    merkle_extras: Vec<MerkleLine>,
    merkle_original: Vec<MerkleLine>,
    merkle_commit: CommitmentLine,
) -> Result<MerkleStatement, ParseError> {
    let qs: Vec<&str> = merkle_extras.iter().map(|n| &n.name[..]).collect();
    let heights: Vec<usize> = merkle_extras
        .iter()
        .map(|mline| mline.node.bits() - 1)
        .collect();

    if !heights.iter().all(|&h| h == heights[0]) {
        return Err(ParseError::InvalidLineFormat);
    }

    let root = U256::from_str_radix(&merkle_commit.digest, 16)?;
    let merkle_queue_values: Vec<U256> = merkle_extras
        .iter()
        .map(|mline| Ok(U256::from_str_radix(&mline.digest, 16)?))
        .collect::<Result<Vec<U256>, ParseError>>()?;
    let proof: Vec<U256> = merkle_original
        .iter()
        .map(|mline| Ok(U256::from_str_radix(&mline.digest, 16)?))
        .collect::<Result<Vec<U256>, ParseError>>()?;
    let merkle_queue_indices: Vec<U256> = merkle_extras.iter().map(|mline| mline.node).collect();

    Ok(MerkleStatement::new(
        root,
        qs.len(),
        heights[0],
        merkle_queue_indices,
        merkle_queue_values,
        proof,
    ))
}

fn montgomery_encode(element: &str) -> Result<U256, ParseError> {
    let prime = BigUint::from_str_radix(
        "800000000000011000000000000000000000000000000000000000000000001",
        16,
    )?;
    let num = BigUint::from_str_radix(element, 16)?;

    let r = BigUint::one() << 256; // Use 2^256 as R
    let encoded: BigUint = (num * r) % prime; // this seems to lost the purpose of montgomery encoding which aims to avoid division

    Ok(U256::from_str_radix(&encoded.to_str_radix(10), 10)?)
}

fn interleave<T: Clone>(a: Vec<T>, b: Vec<T>, c: Vec<T>) -> Vec<T> {
    a.into_iter()
        .zip(b)
        .zip(c)
        .flat_map(|((x, y), z)| vec![x, y, z])
        .collect()
}

/// For a single FRI-Merkle decommitment, processes the information from the non-split proof
/// and extra data prepared by a verifier, and arranges it to be used as input to the
/// FRI-Merkle Fact Registry.
fn gen_fri_merkle_statement_call(
    fri_extras: FriExtras,
    fri_extras_next: FriExtras,
    fri_original: Vec<FriLine>,
    merkle_original: Vec<MerkleLine>,
    merkle_extras: Vec<MerkleLine>,
    merkle_commitment: CommitmentLine,
    evaluation_point: EvalPointLine,
) -> Result<FRIMerkleStatement, ParseError> {
    let root = U256::from_str_radix(&merkle_commitment.digest, 16)?;
    let eval_point = U256::from_str_radix(&evaluation_point.point, 16)?;

    let heights: Vec<usize> = merkle_extras
        .iter()
        .map(|mline| mline.node.bits() - 1)
        .collect();
    assert_eq!(heights.iter().cloned().collect::<HashSet<_>>().len(), 1);
    let output_height = heights[0];

    let mut rows_to_cols: HashMap<usize, Vec<usize>> = HashMap::new();
    for fline in fri_extras.values.iter().chain(fri_original.iter()) {
        rows_to_cols.entry(fline.row).or_default().push(fline.col);
    }
    let row_lens: Vec<usize> = rows_to_cols
        .values()
        .map(|v| v.iter().cloned().collect::<HashSet<_>>().len())
        .collect();
    assert_eq!(row_lens.iter().cloned().collect::<HashSet<_>>().len(), 1);

    let step_size = (row_lens[0] as f64).log2() as usize;
    let input_height = output_height + step_size;

    let input_layer_queries: Vec<U256> = fri_extras
        .inverses
        .iter()
        .map(|fline| U256::from(fline.index + (1 << input_height)))
        .collect();

    let output_layer_queries: Vec<U256> = merkle_extras.iter().map(|mline| mline.node).collect();

    let input_layer_values: Vec<U256> = fri_extras
        .values
        .iter()
        .map(|fline| montgomery_encode(&fline.element))
        .collect::<Result<Vec<U256>, ParseError>>()?;

    let output_layer_values: Vec<U256> = fri_extras_next
        .values
        .iter()
        .map(|fline| montgomery_encode(&fline.element))
        .collect::<Result<Vec<U256>, ParseError>>()?;

    let input_layer_inverses: Vec<U256> = fri_extras
        .inverses
        .iter()
        .map(|fline| Ok(U256::from_str_radix(&fline.inv, 16)?))
        .collect::<Result<Vec<U256>, ParseError>>()?;

    let output_layer_inverses: Vec<U256> = fri_extras_next
        .inverses
        .iter()
        .map(|fline| Ok(U256::from_str_radix(&fline.inv, 16)?))
        .collect::<Result<Vec<U256>, ParseError>>()?;

    let proof: Vec<U256> = fri_original
        .iter()
        .map(|fline| montgomery_encode(&fline.element))
        .chain(
            merkle_original
                .iter()
                .map(|mline| Ok(U256::from_str_radix(&mline.digest, 16)?)),
        )
        .collect::<Result<Vec<U256>, ParseError>>()?;

    let input_interleaved = interleave(
        input_layer_queries.clone(),
        input_layer_values.clone(),
        input_layer_inverses.clone(),
    );
    let output_interleaved = interleave(
        output_layer_queries.clone(),
        output_layer_values.clone(),
        output_layer_inverses.clone(),
    );

    Ok(FRIMerkleStatement {
        expected_root: root,
        evaluation_point: eval_point,
        fri_step_size: step_size,
        input_layer_queries,
        output_layer_queries,
        input_layer_values,
        output_layer_values,
        input_layer_inverses,
        output_layer_inverses,
        // todo: refactor these interleaved into this struct
        input_interleaved,
        output_interleaved,
        proof,
    })
}

/// Parses the extra annotations, returning a dictionary from Merkle name to a list
/// of all its extra (initialization) data.
fn parse_fri_merkles_extra(
    extra_annot_lines: Vec<&str>,
) -> Result<(MerkleExtrasDict, Vec<FriExtras>), ParseError> {
    let mut merkle_extras_dict = MerkleExtrasDict::new();
    let mut fri_extras_dict = HashMap::new();
    let mut fri_names = Vec::new();

    for line in extra_annot_lines {
        if is_merkle_line(line) {
            let mline = parse_merkle_line(line)?;
            merkle_extras_dict
                .entry(mline.name.clone())
                .or_default()
                .push(mline);
        } else if is_fri_line(line) {
            let fline = parse_fri_line(line)?;
            if !fri_extras_dict.contains_key(&fline.name) {
                fri_extras_dict.insert(
                    fline.name.clone(),
                    FriExtras {
                        values: Vec::new(),
                        inverses: Vec::new(),
                    },
                );
                fri_names.push(fline.name.clone());
            }
            fri_extras_dict
                .get_mut(&fline.name)
                .unwrap()
                .values
                .push(fline);
        } else if is_fri_xinv_line(line) {
            let fxline = parse_fri_xinv_line(line)?;
            fri_extras_dict
                .get_mut(&fxline.name)
                .unwrap()
                .inverses
                .push(fxline);
        }
    }
    let fri_extras_list = Vec::from_iter(
        fri_names
            .into_iter()
            .map(|name| fri_extras_dict.remove(&name).unwrap()),
    );

    Ok((merkle_extras_dict, fri_extras_list))
}

/// Parses the original proof annotations, returning a dictionary from Merkle name to
/// a list of all its decommitment data from the proof, another dictionary from Merkle name
/// to the commitments, the trimmed main proof with Merkle decommitments removed,
/// and its annotations.
fn parse_fri_merkles_original(
    orig_proof: Vec<u8>,
    annot_lines: Vec<String>,
) -> Result<FriMerklesOriginal, ParseError> {
    let mut merkle_original_dict = MerkleExtrasDict::new();
    let mut merkle_commits_dict = HashMap::new();
    let mut fri_original_dict = HashMap::new();
    let mut fri_names = Vec::new();
    let mut eval_points_list = Vec::new();
    let mut merkle_patches = HashSet::new();
    let mut main_proof = Vec::new();
    let mut main_annot = String::new();
    let mut trace_commitment_counter = 0;

    for line in annot_lines {
        if is_commitment_line(&line) {
            let (cline, new_trace_commitment_counter) =
                parse_commitment_line(&line, &mut trace_commitment_counter)?;
            merkle_commits_dict.insert(cline.name.clone(), cline);
            trace_commitment_counter = new_trace_commitment_counter;
        } else if is_eval_point_line(&line) {
            let epline = parse_eval_point_line(&line)?;
            eval_points_list.push(epline);
        }

        if is_merkle_line(&line) {
            let mline = parse_merkle_line(&line)?;
            merkle_original_dict
                .entry(mline.name.clone())
                .or_default()
                .push(mline);
        } else if is_merkle_data_line(&line) {
            let mline = parse_merkle_data_line(&line)?;
            let cloned_mline_name = mline.name.clone();
            merkle_original_dict
                .entry(mline.name.clone())
                .or_default()
                .push(mline);
            merkle_patches.insert(cloned_mline_name);
        } else if is_fri_line(&line) {
            let fline = parse_fri_line(&line)?;
            if !fri_original_dict.contains_key(&fline.name) {
                fri_original_dict.insert(fline.name.clone(), Vec::new());
                fri_names.push(fline.name.clone());
            }
            fri_original_dict.get_mut(&fline.name).unwrap().push(fline);
        } else {
            main_annot.push_str(&line);
            main_annot.push('\n');
            let (start, end) = line_to_indices(&line)?;
            main_proof.extend_from_slice(&orig_proof[start..end]);
        }
    }

    Ok(FriMerklesOriginal {
        merkle_originals: merkle_original_dict,
        merkle_commitments: merkle_commits_dict,
        fri_originals: fri_original_dict,
        eval_points: eval_points_list,
        fri_names,
        original_proof: main_proof,
        merkle_patches,
    })
}

/// When any of the traces have a single column, the corresponding Merkle witness is annotated and
/// parsed slightly differently. The hashes in the extra annotations need to be replaced with
/// the field elements (in montgomery form) of the trace itself.
fn single_column_merkle_patch(
    merkle_patches: &HashSet<String>,
    merkle_extras_dict: &mut HashMap<String, Vec<MerkleLine>>,
    annot_lines: &[String],
) -> Result<(), ParseError> {
    for name in merkle_patches {
        let merkle_extras = merkle_extras_dict
            .get(name)
            .ok_or(ParseError::InvalidLineFormat)?
            .clone();
        let heights: Vec<usize> = merkle_extras
            .iter()
            .map(|mline| mline.node.bits() - 1)
            .collect();
        // Ensure all heights are the same
        let height = *heights.first().ok_or(ParseError::InvalidLineFormat)?;
        if !heights.iter().all(|&h| h == height) {
            return Err(ParseError::InvalidLineFormat);
        }
        // When patched, the apparent Merkle height is one lower than the original.
        let height = height + 1;
        merkle_extras_dict.insert(name.clone(), Vec::new());

        for line in annot_lines {
            if line.contains(name) && line.contains("Column 0") && line.contains("Field Element") {
                // It is not a Fri line, but the structure is similar enough for the parser.
                let parsed_fri_line = parse_fri_line(line)?;
                let node = U256::from(parsed_fri_line.row) + U256::from(1 << height);
                let element = montgomery_encode(&parsed_fri_line.element)?;
                let element_hex = format!("{:0>64x}", element);
                let merkle_line = MerkleLine {
                    name: name.clone(),
                    node,
                    digest: element_hex,
                    annotation: line.clone(),
                };
                merkle_extras_dict
                    .get_mut(name)
                    .ok_or(ParseError::InvalidLineFormat)?
                    .push(merkle_line);
            }
        }
    }
    Ok(())
}

/// This is the main function to use to split an [AnnotatedProof] file into a [SplitProofs] file.
/// This processes the annotations of the original proof, and extra annotations prepared by
/// a verifier, and returns a shortened proof (with merkle decommitments removed)
/// in binary form, and a dictionary merkles_statements from the names of the merkle
/// commitments used in the proof to dictionaries containing all data necessary for
/// registering them in the Merkle Fact Registry.
pub fn split_fri_merkle_statements(
    annotated_proof: AnnotatedProof,
) -> Result<SplitProofs, ParseError> {
    // Decode the hexadecimal string
    let orig_proof = hex::decode(&annotated_proof.proof_hex)?;
    let (z, alpha) = annotated_proof.extract_interaction_elements();

    let annot_lines = annotated_proof.annotations;
    let extra_annot_lines = annotated_proof.extra_annotations;

    let (mut merkle_extras_dict, fri_extras_list) =
        parse_fri_merkles_extra(extra_annot_lines.iter().map(|s| s.as_str()).collect())?;
    let fri_merkles_original = parse_fri_merkles_original(orig_proof, annot_lines.clone())?;
    let merkle_names: HashSet<_> = HashSet::from_iter(merkle_extras_dict.keys().cloned());
    assert_eq!(
        merkle_names,
        HashSet::from_iter(fri_merkles_original.merkle_originals.keys().cloned())
    );

    if !fri_merkles_original.merkle_patches.is_empty() {
        single_column_merkle_patch(
            &fri_merkles_original.merkle_patches,
            &mut merkle_extras_dict,
            &annot_lines,
        )?;
    }

    let merkle_statements = merkle_names
        .into_iter()
        .filter(|name| !fri_merkles_original.fri_originals.contains_key(name))
        .map(|name| {
            let statement = gen_merkle_statement_call(
                merkle_extras_dict[&name].clone(),
                fri_merkles_original.merkle_originals[&name].clone(),
                fri_merkles_original.merkle_commitments[&name].clone(),
            )
            .unwrap();
            (name, statement)
        })
        .collect::<HashMap<_, _>>();

    let fri_merkle_statements: Vec<FRIMerkleStatement> = fri_merkles_original
        .fri_names
        .into_iter()
        .enumerate()
        .map(|(i, name)| {
            gen_fri_merkle_statement_call(
                fri_extras_list[i].clone(),
                fri_extras_list[i + 1].clone(),
                fri_merkles_original.fri_originals[&name].clone(),
                fri_merkles_original.merkle_originals[&name].clone(),
                merkle_extras_dict[&name].clone(),
                fri_merkles_original.merkle_commitments[&name].clone(),
                fri_merkles_original.eval_points[i].clone(),
            )
        })
        .collect::<Result<Vec<FRIMerkleStatement>, ParseError>>()?;

    let main_proof = {
        let mut main_proof = fri_merkles_original.original_proof;

        for fri in &fri_merkle_statements[..fri_merkle_statements.len() - 1] {
            let fri_output_interleaved = fri
                .output_interleaved
                .iter()
                .map(|val| Token::Uint(*val))
                .collect();

            let encoded = ethers::abi::encode_packed(&[Token::Array(fri_output_interleaved)])?;
            let hash = keccak256(encoded);
            main_proof.extend_from_slice(&hash);
        }
        main_proof
    };

    let main_proof: MainProof = MainProof::new(
        proof_hex2int_list(main_proof),
        annotated_proof.proof_parameters,
        annotated_proof.public_input,
        z,
        alpha,
    );

    Ok(SplitProofs {
        main_proof,
        merkle_statements,
        fri_merkle_statements,
    })
}

/// Gets a vec of u8 ints and returns it as a 256bits padded list of integer.
/// This conversion is what's needed in order to send a binary proof
/// into an EVM deployed verifier.
fn proof_hex2int_list(proof: Vec<u8>) -> Vec<U256> {
    let chunk_size = 32; // U256 is 32 bytes (256 bits)
    let mut padded_proof = proof;

    // Pad the vector with zeros until its length is a multiple of chunk_size
    while padded_proof.len() % chunk_size != 0 {
        padded_proof.push(0);
    }

    padded_proof
        .chunks(chunk_size)
        .map(|chunk| {
            let mut array = [0u8; 32];
            array.copy_from_slice(chunk);
            U256::from_big_endian(&array)
        })
        .collect()
}
