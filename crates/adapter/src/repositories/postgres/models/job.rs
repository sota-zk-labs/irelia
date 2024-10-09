use std::io::{Error, ErrorKind};
use std::time::SystemTime;

use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use rust_core::entities::job::{JobEntity, JobId};
use uuid::Uuid;

#[derive(Debug, Queryable, Insertable, Selectable, AsChangeset, Identifiable)]
#[diesel(table_name = super::super::schema::jobs)]
pub struct JobModel {
    pub id: Uuid,
    pub customer_id: String,
    pub cairo_job_key: String,
    pub offchain_proof: bool,
    pub proof_layout: String,
    pub cairo_pie: String,

    pub created_on: SystemTime,
}

impl TryFrom<JobEntity> for JobModel {
    type Error = Error;

    fn try_from(entity: JobEntity) -> Result<JobModel, Self::Error> {
        let id = entity
            .id
            .0
            .try_into()
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid ID"))?;

        Ok(JobModel {
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

impl Into<JobEntity> for JobModel {
    fn into(self) -> JobEntity {
        JobEntity {
            id: JobId(self.id.try_into().unwrap()),
            customer_id: self.customer_id,
            cairo_job_key: self.cairo_job_key,
            offchain_proof: self.offchain_proof,
            proof_layout: self.proof_layout,
            cairo_pie: "".to_string(),
        }
    }
}