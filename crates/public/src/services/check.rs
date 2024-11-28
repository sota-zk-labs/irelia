use crate::controllers::worker_job::NewWorkerJob;
use irelia_core::entities::worker_job::WorkerJobStatus;
use irelia_core::entities::worker_job::WorkerJobStatus::{IncorrectLayout, IncorrectOffchainProof, NoCairoJobId, Successfully};

const PROOF_LAYOUT : &str= "small";

pub fn check_worker_job(params: NewWorkerJob) -> (WorkerJobStatus, bool) {
    //Todo: Faulty

    let proof_layout: String = String::from(PROOF_LAYOUT).to_lowercase();
    if params.proof_layout.to_lowercase() !=  proof_layout {
        return (IncorrectLayout, false)
    };

    //Todo: bad flag
    if params.cairo_job_key == None {
        return (NoCairoJobId, false)
    };

    if !params.offchain_proof {
        return (IncorrectOffchainProof, false)
    };

    (Successfully, true)
}
