mod database;
mod printer;

use database::{Queue, CreateQueueRequest, QueueCounts, init_database, get_queue_counts, create_queue, clean_old_queues};
use printer::print_thermal_ticket;
use tauri::{State, Manager};
use std::sync::Arc;
use tokio::sync::Mutex;

type DatabasePool = Arc<Mutex<Option<sqlx::sqlite::SqlitePool>>>;

#[tauri::command]
async fn get_current_queues(pool: State<'_, DatabasePool>) -> Result<QueueCounts, String> {
    let mut pool_guard = pool.lock().await;
    
    // Initialize database if not already done
    if pool_guard.is_none() {
        println!("Initializing database...");
        match init_database().await {
            Ok(db_pool) => {
                *pool_guard = Some(db_pool);
                println!("Database initialized successfully");
                log::info!("Database initialized successfully");
            }
            Err(e) => {
                println!("Failed to initialize database: {}", e);
                log::error!("Failed to initialize database: {}", e);
                return Err(format!("Failed to initialize database: {}", e));
            }
        }
    }
    
    let pool = pool_guard.as_ref().unwrap();
    clean_old_queues(pool).await.map_err(|e| e.to_string())?;
    get_queue_counts(pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_new_queue(pool: State<'_, DatabasePool>, request: CreateQueueRequest) -> Result<Queue, String> {
    let mut pool_guard = pool.lock().await;
    
    // Initialize database if not already done
    if pool_guard.is_none() {
        println!("Initializing database...");
        match init_database().await {
            Ok(db_pool) => {
                *pool_guard = Some(db_pool);
                println!("Database initialized successfully");
                log::info!("Database initialized successfully");
            }
            Err(e) => {
                println!("Failed to initialize database: {}", e);
                log::error!("Failed to initialize database: {}", e);
                return Err(format!("Failed to initialize database: {}", e));
            }
        }
    }
    
    let pool = pool_guard.as_ref().unwrap();
    clean_old_queues(pool).await.map_err(|e| e.to_string())?;
    create_queue(pool, request).await.map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .manage(DatabasePool::new(Mutex::new(None)))
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_current_queues, create_new_queue, print_thermal_ticket])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
