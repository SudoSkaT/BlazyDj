use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: i64,
    pub path: String,
    pub title: Option<String>,
    pub artist: Option<String>,
}

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Open an in-memory SQLite database (for tests and ephemeral use)
    pub async fn open_in_memory() -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect("sqlite::memory:").await?;
        let db = Database { pool };
        db.init().await?;
        Ok(db)
    }

    /// Open a SQLite database at the given connection string (e.g., "sqlite://file.db")
    pub async fn open(conn: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(conn).await?;
        let db = Database { pool };
        db.init().await?;
        Ok(db)
    }

    pub async fn init(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS tracks (id INTEGER PRIMARY KEY, path TEXT NOT NULL, title TEXT, artist TEXT)",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn add_track(
        &self,
        path: &str,
        title: Option<&str>,
        artist: Option<&str>,
    ) -> Result<i64, sqlx::Error> {
        let res = sqlx::query("INSERT INTO tracks (path, title, artist) VALUES (?, ?, ?)")
            .bind(path)
            .bind(title)
            .bind(artist)
            .execute(&self.pool)
            .await?;
        Ok(res.last_insert_rowid())
    }

    pub async fn search_tracks(&self, q: &str) -> Result<Vec<Track>, sqlx::Error> {
        let pattern = format!("%{}%", q);
        let rows = sqlx::query(
            "SELECT id, path, title, artist FROM tracks WHERE title LIKE ? OR artist LIKE ? OR path LIKE ?",
        )
        .bind(&pattern)
        .bind(&pattern)
        .bind(&pattern)
        .fetch_all(&self.pool)
        .await?;

        let mut out = Vec::with_capacity(rows.len());
        for r in rows {
            let id: i64 = r.get::<i64, _>("id");
            let path: String = r.get::<String, _>("path");
            let title: Option<String> = r.get::<Option<String>, _>("title");
            let artist: Option<String> = r.get::<Option<String>, _>("artist");
            out.push(Track {
                id,
                path,
                title,
                artist,
            });
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn add_and_search_track() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::open_in_memory().await?;
        let id = db
            .add_track("/tmp/song.wav", Some("Test Song"), Some("Artist"))
            .await?;
        let found = db.search_tracks("Test Song").await?;
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].id, id);
        assert_eq!(found[0].title.as_deref(), Some("Test Song"));
        Ok(())
    }
}
