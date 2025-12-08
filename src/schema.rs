// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "job_run_status"))]
    pub struct JobRunStatus;
}

diesel::table! {
    channels (id) {
        id -> Varchar,
        name -> Varchar,
        provider_type -> Varchar,
        configuration -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

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

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::JobRunStatus;

    job_runs (id) {
        id -> Uuid,
        application -> Varchar,
        job_name -> Varchar,
        triggered_at -> Timestamptz,
        status -> JobRunStatus,
        stages -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(channels, job_configs, job_runs,);
