use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Test {
    #[serde(alias = "_firestore_id")]
    pub id: Option<String>,
    pub name: String,
    #[serde(with = "firestore::serialize_as_timestamp")]
    pub created_at: DateTime<Utc>,

    #[serde(default)]
    #[serde(with = "firestore::serialize_as_optional_timestamp")]
    pub updated_at: Option<DateTime<Utc>>,
}
