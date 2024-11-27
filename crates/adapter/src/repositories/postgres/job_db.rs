use std::time::SystemTime;
use async_trait::async_trait;
use deadpool_diesel::postgres::Pool;
use diesel::{
    delete, insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use sqlx::types::chrono;
use irelia_core::common::core_error::CoreError;
use irelia_core::entities::job::{Job, JobId, JobPayload, JobStatus};
use irelia_core::ports::job::JobPort;
use crate::repositories::postgres::models::job::JobModel;
use crate::repositories::postgres::schema::jobs::dsl::jobs;
use crate::repositories::postgres::schema::jobs::{cairo_job_key, customer_id, id, updated_on};

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
    async fn add(&self, job: Job) -> Result<Job, CoreError> {
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

    async fn update(&self, job: JobPayload) -> Result<Job, CoreError> {
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

        println!("Hey job_model after update: {:?}", job_model);
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

    async fn get(&self, _customer_id: String, _cairo_job_key: String) -> Result<Job, CoreError> {
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let response = jobs
                    .filter(customer_id.eq(_customer_id))
                    .filter(cairo_job_key.eq(_cairo_job_key))
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
