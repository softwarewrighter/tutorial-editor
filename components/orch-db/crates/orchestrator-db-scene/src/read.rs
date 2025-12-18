//! Scene read operations

use crate::repo::{scene_from_row, SqliteSceneRepository};
use anyhow::{Context, Result};
use async_trait::async_trait;
use orchestrator_domain::Scene;
use orchestrator_ports_scene::SceneReadOps;
use rusqlite::params;

#[async_trait]
impl SceneReadOps for SqliteSceneRepository {
    async fn list_scenes_by_project(&self, project_id: i64) -> Result<Vec<Scene>> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let mut stmt = conn.prepare(
                "SELECT id, project_id, title, description, sort_order, script_text,
                        created_at, updated_at
                 FROM scenes WHERE project_id = ?1 ORDER BY sort_order ASC",
            )?;
            let rows = stmt.query_map(params![project_id], scene_from_row)?;
            rows.collect::<Result<Vec<_>, _>>().context("failed to list scenes")
        })
        .await?
    }

    async fn get_scene(&self, id: i64) -> Result<Option<Scene>> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let mut stmt = conn.prepare(
                "SELECT id, project_id, title, description, sort_order, script_text,
                        created_at, updated_at
                 FROM scenes WHERE id = ?1",
            )?;
            let mut rows = stmt.query(params![id])?;
            match rows.next()? {
                Some(row) => Ok(Some(scene_from_row(row)?)),
                None => Ok(None),
            }
        })
        .await?
    }
}
