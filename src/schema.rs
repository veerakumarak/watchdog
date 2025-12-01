// @generated automatically by Diesel CLI.

diesel::table! {
    job_configs (application, job_name) {
        #[max_length = 255]
        application -> Varchar,
        #[max_length = 255]
        job_name -> Varchar,
        #[max_length = 50]
        schedule -> Nullable<Varchar>,
        #[max_length = 50]
        zone_id -> Nullable<Varchar>,
        enabled -> Bool,
        stages -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
