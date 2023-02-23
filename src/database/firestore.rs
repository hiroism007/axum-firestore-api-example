use firestore::*;

pub async fn initialize_db(project_id: String) -> FirestoreDb {
    FirestoreDb::new(project_id)
        .await
        .expect("Failed to initialize FirestoreDb")
}
