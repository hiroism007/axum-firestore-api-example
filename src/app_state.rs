use firestore::FirestoreDb;

#[derive(Clone)]
pub struct AppState {
    pub db: FirestoreDb,
}

impl AppState {
    pub fn new(db: FirestoreDb) -> Self {
        Self { db }
    }
}
