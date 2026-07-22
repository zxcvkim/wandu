use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::SqlitePool;

use crate::database::QueryResult;

pub type KongId = i64;

#[derive(Debug, Clone, Serialize)]
pub struct Kong {
    pub id: KongId,
    pub profile: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct MiniKong {
    pub id: KongId,
    pub profile: String,
    // pub content: String,
    // pub created_at: NaiveDateTime,
    // pub updated_at: NaiveDateTime,
}

pub async fn create(pool: &SqlitePool, profile: &str, content: &str) -> QueryResult<KongId> {
    sqlx::query_scalar!(
        r#"
            INSERT INTO kongs (profile, content)
            VALUES (?, ?)
            RETURNING id
        "#,
        profile,
        content
    )
    .fetch_one(pool)
    .await
}

pub async fn get(pool: &SqlitePool, id: KongId) -> QueryResult<Option<Kong>> {
    sqlx::query_as!(
        Kong,
        r#"
            SELECT
                id,
                profile,
                content,
                created_at as "created_at: NaiveDateTime",
                updated_at as "updated_at: NaiveDateTime"
            FROM kongs
            WHERE id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn get_all(pool: &SqlitePool) -> QueryResult<Vec<MiniKong>> {
    sqlx::query_as!(
        MiniKong,
        r#"
            SELECT
                id,
                profile
            FROM kongs
            ORDER BY id DESC
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_all_ids(pool: &SqlitePool) -> QueryResult<Vec<KongId>> {
    sqlx::query_scalar!(
        r#"
            SELECT id
            FROM kongs
            ORDER BY id DESC
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_after(pool: &SqlitePool, after: NaiveDateTime) -> QueryResult<Vec<MiniKong>> {
    sqlx::query_as!(
        MiniKong,
        r#"
            SELECT
                id,
                profile
            FROM kongs
            WHERE updated_at > ?
            ORDER BY updated_at ASC
        "#,
        after
    )
    .fetch_all(pool)
    .await
}

pub async fn count(pool: &SqlitePool) -> QueryResult<i64> {
    let result = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM kongs"#)
        .fetch_one(pool)
        .await?;

    Ok(result)
}

pub async fn update(
    pool: &SqlitePool,
    id: KongId,
    profile: Option<&str>,
    content: Option<&str>,
) -> QueryResult<bool> {
    let result = sqlx::query!(
        r#"
            UPDATE kongs
            SET profile = COALESCE(?, profile),
                content = COALESCE(?, content)
            WHERE id = ?
        "#,
        profile,
        content,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn delete(pool: &SqlitePool, id: KongId) -> QueryResult<bool> {
    let result = sqlx::query!(
        r#"
            DELETE FROM kongs
            WHERE id = ?
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
