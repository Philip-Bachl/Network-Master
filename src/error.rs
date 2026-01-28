pub enum Error {
    DatabaseInitError(sqlx::Error),
    PoolConnectionError(sqlx::Error),
    DatabaseReadError(sqlx::Error),
}
