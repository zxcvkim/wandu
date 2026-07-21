mod connect;
pub mod kongs;

pub use connect::connect;

pub type QueryResult<T> = Result<T, sqlx::Error>;
