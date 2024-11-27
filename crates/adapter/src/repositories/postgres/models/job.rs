use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::time::SystemTime;

use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use irelia_core::entities::job::{Job, JobId, JobStatus};
use uuid::Uuid;

#[derive(Debug, Queryable, Insertable, Selectable, AsChangeset, Identifiable, Clone)]
#[diesel(table_name = super::super::schema::jobs)]
pub struct JobModel {
    pub id: Uuid,
    pub customer_id: String,
    pub cairo_job_key: String,
    pub status: String,
    pub validation_done: bool,

    pub updated_on: SystemTime,
    pub created_on: SystemTime,
}

impl TryFrom<Job> for JobModel {
    type Error = Error;

    fn try_from(entity: Job) -> Result<Self, Self::Error> {
        let id = entity
            .id
            .0
            .try_into()
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid ID"))?;
        Ok(JobModel {
            id,
            customer_id: entity.customer_id,
            cairo_job_key: entity.cairo_job_key,
            status: entity.status.to_string(),
            validation_done: entity.validation_done,

            updated_on: SystemTime::now(),
            created_on: SystemTime::now(),
        })
    }
}

impl From<JobModel> for Job {
    fn from(val: JobModel) -> Self {
        Job {
            id: JobId(val.id.try_into().unwrap()),
            customer_id: val.customer_id,
            cairo_job_key: val.cairo_job_key,
            status: JobStatus::from_str(val.status.as_str()).unwrap(),
            validation_done: val.validation_done,
        }
    }
}
