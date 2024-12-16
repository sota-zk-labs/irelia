use async_trait::async_trait;

use crate::common::core_error::CoreError;
use crate::entities::payload_verify_job::PayloadVerifyJob;
use crate::entities::worker_job::WorkerJobEntity;

#[async_trait]
pub trait WorkerPort {
    async fn add(&self, job: WorkerJobEntity) -> Result<WorkerJobEntity, CoreError>;
    async fn verify_merkle(&self, job: PayloadVerifyJob) -> Result<(), CoreError>;
    async fn verify_fri(&self, job: PayloadVerifyJob) -> Result<(), CoreError>;
    async fn register_memory_page(&self, job: PayloadVerifyJob) -> Result<(), CoreError>;
    async fn verify_proof_and_register(&self, job: PayloadVerifyJob) -> Result<(), CoreError>;
}
    