use sqlx::{sqlite::SqlitePool, Row};
use serde::{Deserialize, Serialize};
use chrono::FixedOffset;

#[derive(Debug, Serialize, Deserialize)]
pub struct Queue {
    pub id: i64,
    pub loket_type: String,
    pub queue_number: i32,
    pub queue_code: String,
    pub patient_type: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQueueRequest {
    pub loket_type: String,
    pub patient_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueCounts {
    pub loket_a: i32,
    pub loket_b: i32,
}

pub async fn init_database() -> Result<SqlitePool, sqlx::Error> {
    // Use absolute path for database
    let database_url = "sqlite:./antrian.db";
    println!("Initializing database at: {}", database_url);
    let pool = SqlitePool::connect(database_url).await?;
    
    // Create tables if they don't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS queues (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            loket_type TEXT NOT NULL,
            queue_number INTEGER NOT NULL,
            queue_code TEXT NOT NULL,
            patient_type TEXT NOT NULL,
            status TEXT DEFAULT 'waiting',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;
    
    Ok(pool)
}

// Helper function to get Indonesia time (WIB = UTC+7)
fn indonesia_now() -> chrono::DateTime<FixedOffset> {
    let offset = FixedOffset::east_opt(7 * 3600).unwrap();
    chrono::Utc::now().with_timezone(&offset)
}

pub async fn get_queue_counts(pool: &SqlitePool) -> Result<QueueCounts, sqlx::Error> {
    let today = indonesia_now().format("%Y-%m-%d").to_string();
    
    let loket_a: i32 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM queues WHERE loket_type = 'A' AND DATE(created_at) = ?"
    )
    .bind(&today)
    .fetch_one(pool)
    .await
    .unwrap_or(0);
    
    let loket_b: i32 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM queues WHERE loket_type = 'B' AND DATE(created_at) = ?"
    )
    .bind(&today)
    .fetch_one(pool)
    .await
    .unwrap_or(0);
    
    Ok(QueueCounts {
        loket_a,
        loket_b,
    })
}

pub async fn create_queue(pool: &SqlitePool, request: CreateQueueRequest) -> Result<Queue, sqlx::Error> {
    let today = indonesia_now().format("%Y-%m-%d").to_string();
    
    // Get next queue number for the loket
    let count: i32 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM queues WHERE loket_type = ? AND DATE(created_at) = ?"
    )
    .bind(&request.loket_type)
    .bind(&today)
    .fetch_one(pool)
    .await
    .unwrap_or(0);
    
    let next_number = count + 1;
    let queue_code = format!("{}{:03}", request.loket_type, next_number);
    
    let now = indonesia_now();
    let created_at = now.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
    
    // Insert new queue
    let result = sqlx::query(
        r#"
        INSERT INTO queues (loket_type, queue_number, queue_code, patient_type, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&request.loket_type)
    .bind(next_number)
    .bind(&queue_code)
    .bind(&request.patient_type)
    .bind(&created_at)
    .bind(&created_at)
    .execute(pool)
    .await?;
    
    // Get the created queue
    let row = sqlx::query(
        "SELECT * FROM queues WHERE id = ?"
    )
    .bind(result.last_insert_rowid())
    .fetch_one(pool)
    .await?;
    
    let queue = Queue {
        id: row.get("id"),
        loket_type: row.get("loket_type"),
        queue_number: row.get("queue_number"),
        queue_code: row.get("queue_code"),
        patient_type: row.get("patient_type"),
        status: row.get("status"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    
    Ok(queue)
}

pub async fn clean_old_queues(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let seven_days_ago = indonesia_now() - chrono::Duration::days(7);
    let cutoff_date = seven_days_ago.format("%Y-%m-%d").to_string();
    
    sqlx::query("DELETE FROM queues WHERE DATE(created_at) < ?")
        .bind(&cutoff_date)
        .execute(pool)
        .await?;
    
    Ok(())
}
