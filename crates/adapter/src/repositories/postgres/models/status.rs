use std::io::{Error, ErrorKind};
use std::time::SystemTime;

use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use irelia_core::entities::job::JobId;
use irelia_core::entities::status::{StatusEntity, StatusId};
use uuid::Uuid;

use crate::repositories::postgres::models::job::JobModel;
use crate::repositories::postgres::schema::jobs::{cairo_job_key, customer_id};

#[derive(Debug, Queryable, Insertable, Selectable, AsChangeset, Identifiable, Clone)]
#[diesel(table_name = super::super::schema::job_status)]
pub struct StatusModel {
    pub id: Uuid,
    pub customer_id: String,
    pub cairo_job_key: String,
    pub status: String,
    pub validation_done: bool,

    pub created_on: SystemTime,
}

impl TryFrom<StatusEntity> for StatusModel {
    type Error = Error;

    fn try_from(entity: StatusEntity) -> Result<Self, Self::Error> {
        let id = entity
            .id
            .0
            .try_into()
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid ID"))?;
        Ok(StatusModel {
            id,
            customer_id: entity.customer_id,
            cairo_job_key: entity.cairo_job_key,
            status: entity.status,
            validation_done: entity.validation_done,

            created_on: SystemTime::now(),
        })
    }
}

impl From<StatusModel> for StatusEntity {
    fn from(val: StatusModel) -> Self {
        StatusEntity {
            id: StatusId(val.id.try_into().unwrap()),
            customer_id: val.customer_id,
            cairo_job_key: val.cairo_job_key,
            status: val.status,
            validation_done: val.validation_done,
        }
    }
}
