use std::io::{Error};
use std::time::SystemTime;

use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use irelia_core::entities::job::{JobEntity, JobId};
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
            .0;

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

impl From<JobModel> for JobEntity {
    fn from(val: JobModel) -> Self {
        JobEntity {
            id: JobId(val.id),
            customer_id: val.customer_id,
            cairo_job_key: val.cairo_job_key,
            offchain_proof: val.offchain_proof,
            proof_layout: val.proof_layout,
            cairo_pie: val.cairo_pie,
        }
    }
}
