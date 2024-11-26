use async_trait::async_trait;
use deadpool_diesel::postgres::Pool;
use diesel::{
    delete, insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use irelia_core::common::core_error::CoreError;
use irelia_core::entities::job::{Job, JobId};
use irelia_core::ports::job::JobPort;
use crate::repositories::postgres::models::job::JobModel;
use crate::repositories::postgres::schema::jobs::dsl::jobs;
use crate::repositories::postgres::schema::jobs::id;

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
    async fn add(&self, status_job: Job) -> Result<Job, CoreError> {
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let status_job = JobModel::try_from(status_job)
                    .map_err(|err| CoreError::InternalError(err.into()))?;
                let response = insert_into(jobs)
                    .values(&status_job)
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

    async fn update(&self, status_job: Job) -> Result<Job, CoreError> {
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let status_job = JobModel::try_from(status_job)
                    .map_err(|err| CoreError::InternalError(err.into()))?;
                let response = update(jobs.filter(id.eq(status_job.id)))
                    .set(&status_job)
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

    async fn get(&self, job_id: &JobId) -> Result<Job, CoreError> {
        let job_id = job_id.0;
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let response = jobs
                    .select(JobModel::as_select())
                    .find(job_id)
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
