//! ProjectRepository trait implementation

use crate::mapping::project_from_row;
use crate::struct_def::SqliteProjectRepository;
use anyhow::{Context, Result};
use async_trait::async_trait;
use orchestrator_db_core::format_timestamp;
use orchestrator_domain::Project;
use orchestrator_ports_project::ProjectRepository;
use rusqlite::params;

#[async_trait]
impl ProjectRepository for SqliteProjectRepository {
    async fn list_projects(&self) -> Result<Vec<Project>> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let mut stmt = conn.prepare(
                "SELECT id, slug, title, subtitle, description, created_at, updated_at
                 FROM projects ORDER BY created_at DESC",
            )?;
            let rows = stmt.query_map([], project_from_row)?;
            rows.collect::<Result<Vec<_>, _>>().context("failed to list projects")
        })
        .await?
    }

    async fn get_project(&self, id: i64) -> Result<Option<Project>> {
        let conn = self.conn.clone();
        tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let mut stmt = conn.prepare(
                "SELECT id, slug, title, subtitle, description, created_at, updated_at
                 FROM projects WHERE id = ?1",
            )?;
            let mut rows = stmt.query(params![id])?;
            match rows.next()? {
                Some(row) => Ok(Some(project_from_row(row)?)),
                None => Ok(None),
            }
        })
        .await?
    }

    async fn create_project(&self, mut project: Project) -> Result<Project> {
        let conn = self.conn.clone();
        let p = project.clone();
        let id = tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let created = format_timestamp(p.created_at)?;
            let updated = format_timestamp(p.updated_at)?;
            conn.execute(
                "INSERT INTO projects (slug, title, subtitle, description, created_at, updated_at)
                 VALUES (?1, ?2, NULL, NULL, ?3, ?4)",
                params![p.slug, p.title, created, updated],
            )?;
            Ok::<i64, anyhow::Error>(conn.last_insert_rowid())
        })
        .await??;
        project.id = Some(id);
        Ok(project)
    }
}
