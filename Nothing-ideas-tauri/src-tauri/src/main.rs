// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
use tauri_plugin_sql::{Builder, Migration, MigrationKind};
fn main() {
    // 初始化数据库
    let migrations = vec![
        // Define your migrations here
        Migration {
            version: 1,
            description: "创建nothing_project表",
            sql: "CREATE TABLE nothing_project (
                project_id          INTEGER NOT NULL
                                            PRIMARY KEY AUTOINCREMENT,
                project_create_time DATE,
                project_status      STRING,
                project_title       STRING,
                content_backup      TEXT
            );CREATE TABLE nothing_project_node (
                node_id          INTEGER       NOT NULL
                                               PRIMARY KEY AUTOINCREMENT,
                project_id       INTEGER,
                node_type        VARCHAR (255),
                node_title       VARCHAR (255),
                node_text        TEXT,
                content_backup   TEXT,
                node_create_time DATE
            );CREATE TABLE nothing_project_ref (
                ref_id         INTEGER       NOT NULL
                                             PRIMARY KEY AUTOINCREMENT,
                node_id        INTEGER,
                node_ref_title VARCHAR (255),
                node_ref_link  VARCHAR (255),
                content_backup TEXT
            );",
            kind: MigrationKind::Up,
        }
    ];

    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().add_migrations("sqlite:NothingIdeas.db", migrations).build())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
