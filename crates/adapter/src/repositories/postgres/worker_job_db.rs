use async_trait::async_trait;
use deadpool_diesel::postgres::Pool;
use diesel::{
    delete, insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use irelia_core::common::core_error::CoreError;
use irelia_core::entities::worker_job::{WorkerJob, WorkerJobId};
use irelia_core::ports::worker_job::WorkerJobPort;

use crate::repositories::postgres::models::worker_job::WorkerJobModel;
use crate::repositories::postgres::schema::worker_job::dsl::worker_job;
use crate::repositories::postgres::schema::worker_job::id;

// NOTE: path relative to Cargo.toml
pub const MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("./src/repositories/postgres/migrations");

#[derive(Clone)]
pub struct WorkerJobDBRepository {
    pub db: Pool,
}

impl WorkerJobDBRepository {
    pub fn new(db: Pool) -> Self {
        WorkerJobDBRepository { db }
    }
}

#[async_trait]
impl WorkerJobPort for WorkerJobDBRepository {
    async fn add(&self, job: WorkerJob) -> Result<WorkerJob, CoreError> {
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let job =
                    WorkerJobModel::try_from(job).map_err(|err| CoreError::InternalError(err.into()))?;
                let response = insert_into(worker_job)
                    .values(&job)
                    .get_result::<WorkerJobModel>(conn)
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

    async fn update(&self, job: WorkerJob) -> Result<WorkerJob, CoreError> {
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let job =
                    WorkerJobModel::try_from(job).map_err(|err| CoreError::InternalError(err.into()))?;
                let response = update(worker_job.filter(id.eq(job.id)))
                    .set(&job)
                    .get_result::<WorkerJobModel>(conn)
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

    async fn delete(&self, worker_job_id: &WorkerJobId) -> Result<(), CoreError> {
        let worker_job_id = worker_job_id.0;
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let _ =
                    delete(worker_job.filter(id.eq(worker_job_id)))
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

    async fn get(&self, worker_job_id: &WorkerJobId) -> Result<WorkerJob, CoreError> {
        let worker_job_id = worker_job_id.0;
        self.db
            .get()
            .await
            .unwrap()
            .interact(move |conn| {
                let response = worker_job
                    .select(WorkerJobModel::as_select())
                    .find(worker_job_id)
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

    // async fn get_job_id(&self, params: &Vec<String>) -> Result<WorkerJobId, CoreError> {
    //     let customer_id = &params[0];
    //     let cairo_job_key = &params[1];
    //
    //     self.db.get().await.unwrap().interact(move |conn| {
    //         use crate::repositories::postgres::schema::worker_job::dsl::{worker_job, customer_id as db_customer_id, cairo_job_key as db_cairo_job_key, id};
    //
    //         let worker_job_id: Uuid = worker_job
    //             .filter(db_customer_id.eq(customer_id))
    //             .filter(db_cairo_job_key.eq(cairo_job_key))
    //             .select(id)
    //             .first(conn)
    //             .map_err(|err| match err {
    //                 diesel::result::Error::NotFound => CoreError::NotFound,
    //                 _ => CoreError::InternalError(err.into()),
    //             })?;
    //         Ok(WorkerJobId(worker_job_id))
    //     }).await.unwrap()
    // }
}
