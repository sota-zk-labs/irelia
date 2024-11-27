use std::time::SystemTime;
use async_trait::async_trait;
use deadpool_diesel::postgres::Pool;
use diesel::{
    delete, insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use irelia_core::common::core_error::CoreError;
use irelia_core::entities::job::{JobEntity, JobId, JobEntityPayload};
use irelia_core::ports::job::JobPort;
use crate::repositories::postgres::models::job::JobModel;
use crate::repositories::postgres::schema::jobs::dsl::jobs;
use crate::repositories::postgres::schema::jobs::{cairo_job_key, customer_id, id};

// NOTE: path relative to Cargo.toml

pub const MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("./src/repositories/postgres/migrations");

#[derive(Clone)]
pub struct JobDBRepository {
    pub db: Pool,
}

impl JobDBRepository {
    pub fn new(db: Pool) -> Self {
        JobDBRepository { db }
    }
}

#[async_trait]
impl JobPort for JobDBRepository {
    async fn add(&self, job: JobEntity) -> Result<JobEntity, CoreError> {
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let job_model = JobModel::try_from(job)
                    .map_err(|err| CoreError::InternalError(err.into()))?;
                let response = insert_into(jobs)
                    .values(&job_model)
                    .get_result::<JobModel>(conn)
                    .map_err(|err| match err {
                        diesel::result::Error::NotFound => CoreError::NotFound,
                        _ => CoreError::InternalError(err.into()),
                    })
                    .unwrap();
                Ok(response.into())
            })
            .await
            .unwrap()
    }

    async fn update(&self, job: JobEntityPayload) -> Result<JobEntity, CoreError> {
        let job_model : Result<JobModel, CoreError> = self
            .db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let response = jobs
                    .filter(customer_id.eq(job.customer_id))
                    .filter(cairo_job_key.eq(job.cairo_job_key))
                    .select(JobModel::as_select())
                    .first(conn)
                    .map_err(|err| match err {
                        diesel::result::Error::NotFound => CoreError::NotFound,
                        _ => CoreError::InternalError(err.into()),
                    })?
                    .into();

                Ok(response)
            })
            .await
            .unwrap();

        let mut job_model = job_model?;
        job_model.status = job.status.to_string();
        job_model.validation_done = job.validation_done;
        job_model.updated_on = SystemTime::now();

        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let response = update(jobs.filter(id.eq(job_model.id)))
                    .set(&job_model)
                    .get_result::<JobModel>(conn)
                    .map_err(|err| match err {
                        diesel::result::Error::NotFound => CoreError::NotFound,
                        _ => CoreError::InternalError(err.into()),
                    })?
                    .into();

                Ok(response)
            })
            .await
            .unwrap()
    }

    async fn delete(&self, job_id: &JobId) -> Result<(), CoreError> {
        let job_id = job_id.0;
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let _ = delete(jobs.filter(id.eq(job_id)))
                    .execute(conn)
                    .map_err(|err| match err {
                        diesel::result::Error::NotFound => CoreError::NotFound,
                        _ => CoreError::InternalError(err.into()),
                    })?;

                Ok(())
            })
            .await
            .unwrap()
    }

    async fn get(&self, customer_id_value: String, cairo_job_key_value: String) -> Result<JobEntity, CoreError> {
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let response = jobs
                    .filter(customer_id.eq(customer_id_value))
                    .filter(cairo_job_key.eq(cairo_job_key_value))
                    .select(JobModel::as_select())
                    .first(conn)
                    .map_err(|err| match err {
                        diesel::result::Error::NotFound => CoreError::NotFound,
                        _ => CoreError::InternalError(err.into()),
                    })?
                    .into();

                Ok(response)
            })
            .await
            .unwrap()
    }
}
