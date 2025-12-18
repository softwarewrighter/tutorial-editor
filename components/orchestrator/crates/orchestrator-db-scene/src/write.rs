//! Scene write operations

use crate::repo::SqliteSceneRepository;
use anyhow::Result;
use async_trait::async_trait;
use orchestrator_db_core::format_timestamp;
use orchestrator_domain::Scene;
use orchestrator_ports_scene::SceneWriteOps;
use rusqlite::params;
use time::OffsetDateTime;

#[async_trait]
impl SceneWriteOps for SqliteSceneRepository {
    async fn create_scene(&self, mut scene: Scene) -> Result<Scene> {
        let conn = self.conn.clone();
        let s = scene.clone();
        let id = tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let created = format_timestamp(s.created_at)?;
            let updated = format_timestamp(s.updated_at)?;
            conn.execute(
                "INSERT INTO scenes (project_id, title, description, sort_order,
                                     script_text, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    s.project_id,
                    s.title,
                    s.description,
                    s.sort_order,
                    s.script_text,
                    created,
                    updated
                ],
            )?;
            Ok::<i64, anyhow::Error>(conn.last_insert_rowid())
        })
        .await??;
        scene.id = Some(id);
        Ok(scene)
    }

    async fn update_scene(&self, scene: Scene) -> Result<Scene> {
        let conn = self.conn.clone();
        let s = scene.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let updated = format_timestamp(OffsetDateTime::now_utc())?;
            conn.execute(
                "UPDATE scenes SET title = ?1, description = ?2, sort_order = ?3,
                                   script_text = ?4, updated_at = ?5
                 WHERE id = ?6",
                params![s.title, s.description, s.sort_order, s.script_text, updated, s.id],
            )?;
            Ok::<(), anyhow::Error>(())
        })
        .await??;
        Ok(scene)
    }

    async fn delete_scene(&self, id: i64) -> Result<()> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            conn.execute("DELETE FROM scenes WHERE id = ?1", params![id])?;
            Ok::<(), anyhow::Error>(())
        })
        .await??;
        Ok(())
    }
}
