use firestore::FirestoreDb;

#[derive(Clone)]
pub struct AppState {
    pub db: FirestoreDb,
}
