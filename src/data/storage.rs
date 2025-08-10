use sqlx::{SqlitePool, Row};
use serde_json::Value;
use std::collections::HashMap;
use tokio::fs;
use crate::Result;

pub struct Storage {
    pool: SqlitePool,
}

impl Storage {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        
        // Create tables if they don't exist
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS scraped_data (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                url TEXT NOT NULL,
                data TEXT NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                task_id TEXT
            )
        "#)
        .execute(&pool)
        .await?;

        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                query TEXT NOT NULL,
                status TEXT NOT NULL,
                result TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                completed_at DATETIME
            )
        "#)
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub async fn store_scraped_data(&self, url: &str, data: &Value, task_id: Option<&str>) -> Result<i64> {
        let data_json = serde_json::to_string(data)?;
        
        let result = sqlx::query(r#"
            INSERT INTO scraped_data (url, data, task_id) 
            VALUES (?, ?, ?)
        "#)
        .bind(url)
        .bind(data_json)
        .bind(task_id)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_scraped_data(&self, limit: Option<i64>) -> Result<Vec<(String, Value)>> {
        let limit = limit.unwrap_or(100);
        
        let rows = sqlx::query(r#"
            SELECT url, data FROM scraped_data 
            ORDER BY timestamp DESC 
            LIMIT ?
        "#)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut results = Vec::new();
        for row in rows {
            let url: String = row.get("url");
            let data_str: String = row.get("data");
            let data: Value = serde_json::from_str(&data_str)?;
            results.push((url, data));
        }

        Ok(results)
    }

    pub async fn store_task(&self, task_id: &str, query: &str, status: &str) -> Result<()> {
        sqlx::query(r#"
            INSERT OR REPLACE INTO tasks (id, query, status) 
            VALUES (?, ?, ?)
        "#)
        .bind(task_id)
        .bind(query)
        .bind(status)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_task_status(&self, task_id: &str, status: &str, result: Option<&str>) -> Result<()> {
        let completed_at = if status == "completed" || status == "failed" {
            Some(chrono::Utc::now().naive_utc())
        } else {
            None
        };

        sqlx::query(r#"
            UPDATE tasks 
            SET status = ?, result = ?, completed_at = ?
            WHERE id = ?
        "#)
        .bind(status)
        .bind(result)
        .bind(completed_at)
        .bind(task_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_task(&self, task_id: &str) -> Result<Option<(String, String, String)>> {
        let row = sqlx::query(r#"
            SELECT query, status, result FROM tasks WHERE id = ?
        "#)
        .bind(task_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let query: String = row.get("query");
            let status: String = row.get("status");
            let result: Option<String> = row.get("result");
            Ok(Some((query, status, result.unwrap_or_default())))
        } else {
            Ok(None)
        }
    }

    pub async fn clean_old_data(&self, days: i64) -> Result<u64> {
        let result = sqlx::query(r#"
            DELETE FROM scraped_data 
            WHERE timestamp < datetime('now', '-' || ? || ' days')
        "#)
        .bind(days)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
