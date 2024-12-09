// @generated automatically by Diesel CLI.
diesel::table! {
    jobs(id) {
        id -> Uuid,
        #[max_length = 255]
        customer_id -> Varchar,
        cairo_job_key -> Varchar,
        status -> Varchar,
        invalid_reason -> Varchar,
        error_log -> Varchar,
        validation_done -> Bool,

        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}
