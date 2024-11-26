use std::io::{Error, ErrorKind};
use std::time::SystemTime;

use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use irelia_core::entities::worker_job::{WorkerJob, JobId};
use uuid::Uuid;

#[derive(Debug, Queryable, Insertable, Selectable, AsChangeset, Identifiable)]
#[diesel(table_name = super::super::schema::jobs)]
pub struct WorkedJobModel {
    pub id: Uuid,
    pub customer_id: String,
    pub cairo_job_key: String,
    pub offchain_proof: bool,
    pub proof_layout: String,
    pub cairo_pie: String,

    pub created_on: SystemTime,
}

impl TryFrom<WorkerJob> for WorkedJobModel {
    type Error = Error;

    fn try_from(entity: WorkerJob) -> Result<WorkedJobModel, Self::Error> {
        let id = entity
            .id
            .0
            .try_into()
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid ID"))?;

        Ok(WorkedJobModel {
            id,
            customer_id: entity.customer_id,
            cairo_job_key: entity.cairo_job_key,
            offchain_proof: entity.offchain_proof,
            proof_layout: entity.proof_layout,
            cairo_pie: entity.cairo_pie,

            created_on: SystemTime::now(),
        })
    }
}

impl From<WorkedJobModel> for WorkerJob {
    fn from(val: WorkedJobModel) -> Self {
        WorkerJob {
            id: JobId(val.id.try_into().unwrap()),
            customer_id: val.customer_id,
            cairo_job_key: val.cairo_job_key,
            offchain_proof: val.offchain_proof,
            proof_layout: val.proof_layout,
            cairo_pie: "".to_string(),
        }
    }
}
