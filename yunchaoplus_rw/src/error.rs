use thiserror::Error;

#[derive(Debug, Error)]
pub enum DBError {
    #[error("DB Pool error")]
    Pool(#[from] deadpool_postgres::PoolError),
    #[error("Database error")]
    Postgres(#[from] tokio_postgres::Error),
}
