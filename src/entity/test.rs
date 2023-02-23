use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Test {
    #[serde(alias = "_firestore_id")]
    pub id: Option<String>,
    pub name: String,
    #[serde(alias = "_firestore_updated")]
    pub created_at: Option<DateTime<Utc>>,
}
