use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkFormData {
    path: String,
    destination: String,
}

impl LinkFormData {
    pub fn new(path: String, destination: String) -> Self {
        LinkFormData { path, destination }
    }
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Link {
    pub id: Uuid,
    pub path: String,
    pub destination: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl Link {
    pub fn from_form_data(form: LinkFormData) -> Self {
        let now = Utc::now();
        Link {
            id: Uuid::new_v4(),
            path: form.path,
            destination: form.destination,
            created_at: now,
            modified_at: now,
        }
    }

    pub async fn fetch_all(pg: &PgPool) -> sqlx::Result<Vec<Link>> {
        query_as!(
            Link,
            r#"
        SELECT *
        FROM links
        "#
        )
        .fetch_all(pg)
        .await
    }

    pub async fn upsert(&self, pg: &PgPool) -> sqlx::Result<PgQueryResult> {
        query!(
            r#"
        INSERT INTO links (id, path, destination, created_at, modified_at)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (path) DO UPDATE
        SET destination = EXCLUDED.destination, modified_at = EXCLUDED.modified_at
        "#,
            self.id,
            self.path,
            self.destination,
            self.created_at,
            self.modified_at
        )
        .execute(pg)
        .await
    }

    pub async fn fetch_by_path(path: &str, pg: &PgPool) -> sqlx::Result<Link> {
        query_as!(
            Link,
            r#"
        SELECT *
        FROM links
        WHERE path = $1
        "#,
            path
        )
        .fetch_one(pg)
        .await
    }

    pub async fn delete_by_path(path: &str, pg: &PgPool) -> sqlx::Result<PgQueryResult> {
        query!(
            r#"
        DELETE FROM links
        WHERE path = $1
        "#,
            path
        )
        .execute(pg)
        .await
    }
}
