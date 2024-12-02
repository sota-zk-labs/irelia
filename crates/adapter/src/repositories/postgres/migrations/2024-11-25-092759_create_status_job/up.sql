-- Your SQL goes here
CREATE TABLE jobs (
                            id UUID PRIMARY KEY,
                            customer_id VARCHAR NOT NULL,
                            cairo_job_key VARCHAR NOT NULL,
                            status VARCHAR NOT NULL,
                            invalid_reason VARCHAR NOT NULL,
                            error_log VARCHAR NOT NULL,
                            validation_done BOOLEAN NOT NULL,
                            created_on TIMESTAMP NOT NULL DEFAULT NOW(),
                            updated_on TIMESTAMP NOT NULL DEFAULT NOW()
);
