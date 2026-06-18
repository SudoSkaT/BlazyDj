use anyhow::Result;
use lazydj_database::Database;
use std::path::Path;

/// Import a single file into the provided database. This is intentionally minimal
/// for Phase 3: extract a simple title from the file stem and record the path.
pub async fn import_file(db: &Database, path: &str) -> Result<i64> {
    let p = Path::new(path);
    let title = p
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string());
    let title_ref = title.as_deref();
    let id = db
        .add_track(path, title_ref, None)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn import_file_and_find() -> Result<(), Box<dyn std::error::Error>> {
        let db = lazydj_database::Database::open_in_memory().await?;
        let id = import_file(&db, "/tmp/some-track.wav").await?;
        let results = db.search_tracks("some-track").await?;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, id);
        Ok(())
    }
}
