use crate::controllers::worker_job::NewWorkerJob;
use irelia_core::entities::worker_job::WorkerJobStatus::{
    AdditionalBadFlag, FaultyCairoPie, IncorrectLayout, IncorrectOffchainProof, NoCairoJobId,
    Successfully,
};
use irelia_core::entities::worker_job::{ProofLayout, WorkerJobStatus};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::PathBuf, str::FromStr};

pub fn check_worker_job(params: NewWorkerJob, cairo_pie: String) -> (WorkerJobStatus, bool) {
    // Check faulty cairo pie
    let cairo_pie_path = PathBuf::from(cairo_pie);
    match File::open(&cairo_pie_path) {
        Ok(file) => match zip::ZipArchive::new(file) {
            Ok(_) => (),
            _ => return (FaultyCairoPie, false),
        },
        _ => return (FaultyCairoPie, false),
    }

    // Check incorrect layout
    match ProofLayout::from_str(params.proof_layout.to_lowercase().as_str()) {
        Ok(_) => (),
        _ => {
            return (IncorrectLayout, false);
        }
    }
    // Check additional bad flag
    if params.bla != None && params.bla.unwrap() == true {
        return (AdditionalBadFlag, true);
    }

    // Check no cairo job id
    if params.cairo_job_key == None {
        return (NoCairoJobId, false);
    };

    // check incorrect off chain proof
    if !params.offchain_proof {
        return (IncorrectOffchainProof, false);
    };

    (Successfully, true)
}

#[cfg(test)]
mod test {
    use crate::controllers::worker_job::NewWorkerJob;
    use crate::services::check::check_worker_job;

    #[test]
    fn test_check() {
        let params = NewWorkerJob {
            customer_id: "1".to_string(),
            cairo_job_key: Some("1".to_string()),
            offchain_proof: true,
            proof_layout: "starknet".to_string(),
            bla: None,
        };
        let cairo_pie = "/home/andrew/workspace/irelia/crates/\
                    adapter/src/prover/test_samples/fibonacci_with_output.zip"
            .to_string();
        let (response_code, is_valid) = check_worker_job(params, cairo_pie);
        println!("response_code: {:?}", response_code);
        println!("is_valid: {:?}", is_valid)
    }
}
