use std::io::Error;
use std::str::FromStr;
use std::time::SystemTime;

use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use irelia_core::entities::job::{CairoJobStatus, JobEntity, JobId};
use uuid::Uuid;

#[derive(Debug, Queryable, Insertable, Selectable, AsChangeset, Identifiable, Clone)]
#[diesel(table_name = super::super::schema::jobs)]
pub struct JobModel {
    pub id: Uuid,
    pub customer_id: String,
    pub cairo_job_key: String,
    pub status: String,
    pub invalid_reason: String,
    pub error_log: String,
    pub validation_done: bool,

    pub updated_on: SystemTime,
    pub created_on: SystemTime,
}

impl TryFrom<JobEntity> for JobModel {
    type Error = Error;

    fn try_from(entity: JobEntity) -> Result<Self, Self::Error> {
        Ok(JobModel {
            id: entity.id.0,
            customer_id: entity.customer_id,
            cairo_job_key: entity.cairo_job_key,
            status: entity.status.to_string(),
            invalid_reason: entity.invalid_reason,
            error_log: entity.error_log,
            validation_done: entity.validation_done,

            updated_on: SystemTime::now(),
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
            status: CairoJobStatus::from_str(val.status.as_str()).unwrap(),
            invalid_reason: val.invalid_reason,
            error_log: val.error_log,
            validation_done: val.validation_done,
        }
    }
}
