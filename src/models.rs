use std::collections::HashMap;
use chrono::{DateTime, Utc};
use diesel::{serialize, Queryable};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::*;

use diesel::sql_types::Jsonb;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use serde_json;
use std::error::Error;
use diesel::expression::AsExpression;
use diesel::serialize::ToSql;
// use diesel::expression::bound::Bound;
use diesel::expression::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobStageConfig {
    pub name: String,
    pub start: Option<u64>,
    pub complete: Option<u64>,
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct JsonbStages(pub Vec<JobStageConfig>);
//
// impl AsExpression<Jsonb> for JsonbStages {
//     type Expression = diesel::expression::bound::Bound<Jsonb, Self>;
//
//     fn as_expression(self) -> Self::Expression {
//         Bound::new(self)
//     }
// }
// impl<'a> AsExpression<Jsonb> for &'a JsonbStages {
//     type Expression = Bound<Jsonb, Self>;
//
//     fn as_expression(self) -> Self::Expression {
//         Bound::new(self)
//     }
// }
//
// impl FromSql<Jsonb, Pg> for JsonbStages {
//     fn from_sql(bytes: diesel::backend::RawValue<'_, Pg>) -> deserialize::Result<Self> {
//         let json_value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
//
//         // 2. Deserialize the serde_json::Value into Vec<JobStageConfig> (the inner type)
//         let inner_vec: Vec<JobStageConfig> = serde_json::from_value(json_value)
//             .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
//
//         // 3. Wrap the result in the newtype struct
//         Ok(JsonbStages(inner_vec))
//     }
// }
//
// impl ToSql<Jsonb, Pg> for JsonbStages {
//     fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
//         // 1. Serialize the INNER FIELD (self.0) into a serde_json::Value
//         let value = serde_json::to_value(&self.0)?; // <--- Use self.0
//
//         // 2. Delegate the writing to the existing ToSql implementation for serde_json::Value
//         <serde_json::Value as ToSql<Jsonb, Pg>>::to_sql(&value, &mut out.reborrow())
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, AsChangeset)]
#[diesel(table_name = job_configs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(application, job_name))]
pub struct JobConfig {
    pub application: String,
    pub job_name: String,
    pub schedule: Option<String>,
    pub zone_id: Option<String>,
    pub enabled: bool,
    pub stages: diesel_json::Json<Vec<JobStageConfig>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = job_configs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewJobConfig {
    pub application: String,
    pub job_name: String,
    pub schedule: Option<String>,
    pub zone_id: Option<String>,
    pub stages: diesel_json::Json<Vec<JobStageConfig>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobRun {
    pub run_id: String, // Partition Key
    pub job_name: String,
    pub start_time: u64, // Unix timestamp
    // Map of "stage_name" -> completion_timestamp
    pub completed_stages: HashMap<String, u64>,
    pub is_active: bool,
}
