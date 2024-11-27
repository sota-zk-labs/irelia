// @generated automatically by Diesel CLI.
diesel::table! {
    jobs(id) {
        id -> Uuid,
        customer_id -> Varchar,
        cairo_job_key -> Varchar,
        status -> Varchar,
        validation_done -> Bool,

        updated_on -> Timestamp,
        created_on -> Timestamp,
    }
}
