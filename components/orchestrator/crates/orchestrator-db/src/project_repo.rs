use crate::schema::init_db;
use anyhow::{Context, Result};
use async_trait::async_trait;
use orchestrator_core::domain::Project;
use orchestrator_core::ports::ProjectRepository;
use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};
use time::OffsetDateTime;

#[derive(Clone)]
pub struct SqliteProjectRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteProjectRepository {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)
            .with_context(|| format!("failed to open sqlite db at {path}"))?;
        init_db(&conn)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
}

#[async_trait]
impl ProjectRepository for SqliteProjectRepository {
    async fn list_projects(&self) -> Result<Vec<Project>> {
        let conn = self.conn.clone();
        let projects = tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let mut stmt = conn.prepare(
                "SELECT id, slug, title, subtitle, description, created_at, updated_at FROM projects ORDER BY created_at DESC",
            )?;
            let rows = stmt.query_map([], |row| {
                let created_at_str: String = row.get(5)?;
                let updated_at_str: String = row.get(6)?;
                let created_at = OffsetDateTime::parse(&created_at_str, &time::format_description::well_known::Rfc3339)
                    .unwrap_or_else(|_| OffsetDateTime::now_utc());
                let updated_at = OffsetDateTime::parse(&updated_at_str, &time::format_description::well_known::Rfc3339)
                    .unwrap_or_else(|_| OffsetDateTime::now_utc());
                Ok(Project {
                    id: row.get(0)?,
                    slug: row.get(1)?,
                    title: row.get(2)?,
                    subtitle: row.get(3)?,
                    description: row.get(4)?,
                    created_at,
                    updated_at,
                })
            })?;

            let mut out = Vec::new();
            for p in rows {
                out.push(p?);
            }
            Ok::<_, anyhow::Error>(out)
        })
        .await??;

        Ok(projects)
    }

    async fn create_project(&self, mut project: Project) -> Result<Project> {
        let conn = self.conn.clone();
        let slug = project.slug.clone();
        let title = project.title.clone();
        let created_at = project.created_at;
        let updated_at = project.updated_at;

        let created = tokio::task::spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            let created_at_str =
                created_at.format(&time::format_description::well_known::Rfc3339)?;
            let updated_at_str =
                updated_at.format(&time::format_description::well_known::Rfc3339)?;
            conn.execute(
                "INSERT INTO projects (slug, title, subtitle, description, created_at, updated_at)
                 VALUES (?1, ?2, NULL, NULL, ?3, ?4)",
                params![slug, title, created_at_str, updated_at_str],
            )?;
            let id = conn.last_insert_rowid();
            Ok::<i64, anyhow::Error>(id)
        })
        .await??;

        project.id = Some(created);
        Ok(project)
    }
}
