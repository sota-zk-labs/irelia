// @generated automatically by Diesel CLI.

diesel::table! {
    jobs (id) {
        id -> Uuid,
        #[max_length = 255]
        customer_id -> Varchar,
        cairo_job_key -> Varchar,
        offchain_proof -> Bool,
        proof_layout -> Varchar,
        cairo_pie -> Text,
        created_on -> Timestamp,
    }
}
