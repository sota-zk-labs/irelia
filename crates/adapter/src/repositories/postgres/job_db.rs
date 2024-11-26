use async_trait::async_trait;
use deadpool_diesel::postgres::Pool;
use diesel::dsl::select;
use diesel::{
    delete, insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use irelia_core::common::core_error::CoreError;
use irelia_core::entities::job::Job;
use irelia_core::ports::job::StatusPort;
use uuid::Uuid;
use irelia_core::entities::worker_job::JobId;
use crate::repositories::postgres::models::status::StatusModel;
use crate::repositories::postgres::schema::job_status::dsl::job_status;
use crate::repositories::postgres::schema::job_status::id;

// NOTE: path relative to Cargo.toml

pub const MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("./src/repositories/postgres/migrations");

#[derive(Clone)]
pub struct StatusJobDBRepository {
    pub db: Pool,
}

impl StatusJobDBRepository {
    pub fn new(db: Pool) -> Self {
        StatusJobDBRepository { db }
    }
}

#[async_trait]
impl StatusPort for StatusJobDBRepository {
    async fn add(&self, status_job: Job) -> Result<Job, CoreError> {
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let status_job = StatusModel::try_from(status_job)
                    .map_err(|err| CoreError::InternalError(err.into()))?;
                let response = insert_into(job_status)
                    .values(&status_job)
                    .get_result::<StatusModel>(conn)
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
                let status_job = StatusModel::try_from(status_job)
                    .map_err(|err| CoreError::InternalError(err.into()))?;
                let response = update(job_status.filter(id.eq(status_job.id)))
                    .set(&status_job)
                    .get_result::<StatusModel>(conn)
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

    async fn delete(&self, status_job_id: &JobId) -> Result<(), CoreError> {
        let status_job_id = status_job_id.0;
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let _ = delete(job_status.filter(id.eq(status_job_id)))
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

    async fn get(&self, status_job_id: &JobId) -> Result<Job, CoreError> {
        let status_job_id = status_job_id.0;
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let response = job_status
                    .select(StatusModel::as_select())
                    .find(status_job_id)
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
