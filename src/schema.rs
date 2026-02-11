// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "job_run_status"))]
    pub struct JobRunStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "provider_type"))]
    pub struct ProviderType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ProviderType;

    channels (name) {
        name -> Varchar,
        provider_type -> ProviderType,
        configuration -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    global_settings (id) {
        id -> Int4,
        success_retention_days -> Int4,
        failure_retention_days -> Int4,
        maintenance_mode -> Bool,
        default_channels -> Text,
        error_channels -> Text,
        max_stage_duration_hours -> Int4,
    }
}

diesel::table! {
    job_configs (app_name, job_name) {
        #[max_length = 255]
        app_name -> Varchar,
        #[max_length = 255]
        job_name -> Varchar,
        #[max_length = 50]
        schedule -> Nullable<Varchar>,
        #[max_length = 50]
        zone_id -> Nullable<Varchar>,
        enabled -> Bool,
        stages -> Jsonb,
        channel_ids -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::JobRunStatus;

    job_runs (id) {
        id -> Uuid,
        app_name -> Varchar,
        job_name -> Varchar,
        triggered_at -> Timestamptz,
        status -> JobRunStatus,
        stages -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(channels, global_settings, job_configs, job_runs,);
