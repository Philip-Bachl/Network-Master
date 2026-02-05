#[derive(Debug)]
pub enum MasterbaseError {
    DatabaseInit(sqlx::Error),
    DatabaseSeed(sqlx::Error),
}
