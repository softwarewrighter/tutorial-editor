use crate::scene_mapping;
use crate::timestamps;
use anyhow::{Context, Result};
use async_trait::async_trait;
use orchestrator_core::domain::Scene;
use orchestrator_core::ports::SceneRepository;
use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};
use time::OffsetDateTime;

#[derive(Clone)]
pub struct SqliteSceneRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteSceneRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl SceneRepository for SqliteSceneRepository {
    async fn list_scenes_by_project(&self, project_id: i64) -> Result<Vec<Scene>> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let mut stmt = conn.prepare(
                "SELECT id, project_id, title, description, sort_order, script_text,
                        created_at, updated_at
                 FROM scenes WHERE project_id = ?1 ORDER BY sort_order ASC",
            )?;
            let rows = stmt.query_map(params![project_id], scene_mapping::from_row)?;
            rows.collect::<Result<Vec<_>, _>>()
                .context("failed to collect scenes")
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
                Some(row) => Ok(Some(scene_mapping::from_row(row)?)),
                None => Ok(None),
            }
        })
        .await?
    }

    async fn create_scene(&self, mut scene: Scene) -> Result<Scene> {
        let conn = self.conn.clone();
        let scene_clone = scene.clone();
        let id = tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let created_str = timestamps::format(scene_clone.created_at)?;
            let updated_str = timestamps::format(scene_clone.updated_at)?;
            conn.execute(
                "INSERT INTO scenes (project_id, title, description, sort_order,
                                     script_text, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    scene_clone.project_id,
                    scene_clone.title,
                    scene_clone.description,
                    scene_clone.sort_order,
                    scene_clone.script_text,
                    created_str,
                    updated_str
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
        let scene_clone = scene.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let updated_str = timestamps::format(OffsetDateTime::now_utc())?;
            conn.execute(
                "UPDATE scenes SET title = ?1, description = ?2, sort_order = ?3,
                                   script_text = ?4, updated_at = ?5
                 WHERE id = ?6",
                params![
                    scene_clone.title,
                    scene_clone.description,
                    scene_clone.sort_order,
                    scene_clone.script_text,
                    updated_str,
                    scene_clone.id
                ],
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
