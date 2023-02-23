use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct User {
    #[serde(alias = "_firestore_id")]
    id: Option<String>,
    client_id: String,
    #[serde(alias = "_firestore_updated")]
    last_login_at: Option<DateTime<Utc>>,
}
