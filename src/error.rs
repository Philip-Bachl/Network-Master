pub enum Error {
    DatabaseInitError(sqlx::Error),
}
