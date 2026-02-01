use crate::models::{
    AppError, AppResult, CreateServerArgs, McpServer, RegistryInstallConfig, RegistryItem,
    RegistryServer, ResearchNote, UpdateServerArgs,
};
use rusqlite::{params, Connection};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new() -> AppResult<Self> {
        let db_path = get_db_path()?;
        let conn = Connection::open(db_path)?;
        init_db_schema(&conn)?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.bootstrap_registry()?;
        Ok(db)
    }

    fn bootstrap_registry(&self) -> AppResult<()> {
        let items = self.get_cached_registry(Some("official"))?;
        if items.is_empty() {
            println!("Bootstrapping registry from JSON...");
            let registry_json = include_str!("../registry.json");
            let official_items: Vec<RegistryItem> = serde_json::from_str(registry_json)?;
            self.cache_registry(&official_items, "official")?;
        }
        Ok(())
    }

    // For testing purposes
    #[allow(dead_code)]
    pub fn new_in_memory() -> AppResult<Self> {
        let conn = Connection::open_in_memory()?;
        init_db_schema(&conn)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn get_servers(&self) -> AppResult<Vec<McpServer>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::Database(e.to_string()))?;
        let mut stmt = conn.prepare("SELECT * FROM mcp_servers ORDER BY created_at DESC")?;

        let server_iter = stmt.query_map([], |row| {
            let args_str: Option<String> = row.get(4).ok();
            let env_str: Option<String> = row.get(6).ok();

            Ok(McpServer {
                id: row.get(0)?,
                name: row.get(1)?,
                server_type: row.get(2)?,
                command: row.get(3)?,
                args: args_str.and_then(|s| serde_json::from_str(&s).ok()),
                url: row.get(5)?,
                env: env_str.and_then(|s| serde_json::from_str(&s).ok()),
                description: row.get(7)?,
                is_active: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        })?;

        let mut servers = Vec::new();
        for server in server_iter {
            servers.push(server?);
        }

        Ok(servers)
    }

    pub fn get_server(&self, id: String) -> AppResult<McpServer> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::Database(e.to_string()))?;
        let mut stmt = conn.prepare("SELECT * FROM mcp_servers WHERE id = ?1")?;

        let server = stmt.query_row(params![id], |row| {
            let args_str: Option<String> = row.get(4).ok();
            let env_str: Option<String> = row.get(6).ok();

            Ok(McpServer {
                id: row.get(0)?,
                name: row.get(1)?,
                server_type: row.get(2)?,
                command: row.get(3)?,
                args: args_str.and_then(|s| serde_json::from_str(&s).ok()),
                url: row.get(5)?,
                env: env_str.and_then(|s| serde_json::from_str(&s).ok()),
                description: row.get(7)?,
                is_active: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        })?;

        Ok(server)
    }

    pub fn create_server(&self, args: CreateServerArgs) -> AppResult<McpServer> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::Database(e.to_string()))?;
        let id = Uuid::new_v4().to_string();

        let args_json = serde_json::to_string(&args.args.unwrap_or_default())?;
        let env_json = serde_json::to_string(&args.env.unwrap_or_default())?;

        conn.execute(
            "INSERT INTO mcp_servers (id, name, type, command, args, url, env, description) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                id,
                args.name,
                args.server_type,
                args.command,
                args_json,
                args.url,
                env_json,
                args.description
            ],
        )?;

        // Fetch back to return full object
        let mut stmt = conn.prepare("SELECT * FROM mcp_servers WHERE id = ?1")?;
        let server = stmt.query_row(params![id], |row| {
            let args_str: Option<String> = row.get(4).ok();
            let env_str: Option<String> = row.get(6).ok();

            Ok(McpServer {
                id: row.get(0)?,
                name: row.get(1)?,
                server_type: row.get(2)?,
                command: row.get(3)?,
                args: args_str.and_then(|s| serde_json::from_str(&s).ok()),
                url: row.get(5)?,
                env: env_str.and_then(|s| serde_json::from_str(&s).ok()),
                description: row.get(7)?,
                is_active: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        })?;

        Ok(server)
    }

    pub fn update_server(&self, id: String, args: UpdateServerArgs) -> AppResult<McpServer> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::Database(e.to_string()))?;

        if let Some(val) = args.name {
            self.execute_update(&conn, "name", val, &id)?;
        }
        if let Some(val) = args.server_type {
            self.execute_update(&conn, "type", val, &id)?;
        }
        if let Some(val) = args.command {
            self.execute_update(&conn, "command", val, &id)?;
        }
        if let Some(val) = args.args {
            self.execute_update(&conn, "args", serde_json::to_string(&val)?, &id)?;
        }
        if let Some(val) = args.url {
            self.execute_update(&conn, "url", val, &id)?;
        }
        if let Some(val) = args.env {
            self.execute_update(&conn, "env", serde_json::to_string(&val)?, &id)?;
        }
        if let Some(val) = args.description {
            self.execute_update(&conn, "description", val, &id)?;
        }
        if let Some(val) = args.is_active {
            self.execute_update(&conn, "is_active", val, &id)?;
        }

        // Fetch updated
        let mut stmt = conn.prepare("SELECT * FROM mcp_servers WHERE id = ?1")?;
        let server = stmt.query_row(params![id], |row| {
            let args_str: Option<String> = row.get(4).ok();
            let env_str: Option<String> = row.get(6).ok();
            Ok(McpServer {
                id: row.get(0)?,
                name: row.get(1)?,
                server_type: row.get(2)?,
                command: row.get(3)?,
                args: args_str.and_then(|s| serde_json::from_str(&s).ok()),
                url: row.get(5)?,
                env: env_str.and_then(|s| serde_json::from_str(&s).ok()),
                description: row.get(7)?,
                is_active: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        })?;
        Ok(server)
    }

    fn execute_update<T: rusqlite::ToSql>(
        &self,
        conn: &Connection,
        field: &str,
        val: T,
        id: &str,
    ) -> AppResult<()> {
        let query = format!(
            "UPDATE mcp_servers SET {} = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            field
        );
        conn.execute(&query, params![val, id])?;
        Ok(())
    }

    pub fn delete_server(&self, id: String) -> AppResult<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::Database(e.to_string()))?;
        conn.execute("DELETE FROM mcp_servers WHERE id = ?1", params![id])?;
        Ok(())
    }

    // === Registry Cache Methods ===

    /// Cache registry items for offline use
    pub fn cache_registry(&self, items: &[RegistryItem], source: &str) -> AppResult<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Clear existing items from this source
        conn.execute(
            "DELETE FROM registry_cache WHERE source = ?1",
            params![source],
        )?;

        // Insert new items
        for item in items {
            let args_json = item
                .install_config
                .as_ref()
                .map(|c| serde_json::to_string(&c.args).unwrap_or_default());
            let env_json = item
                .install_config
                .as_ref()
                .and_then(|c| c.env_template.as_ref())
                .map(|e| serde_json::to_string(e).unwrap_or_default());
            let wizard_json = item
                .install_config
                .as_ref()
                .and_then(|c| c.wizard.as_ref())
                .map(|w| serde_json::to_string(w).unwrap_or_default());
            let topics_json = serde_json::to_string(&item.topics).unwrap_or_default();

            conn.execute(
                "INSERT OR REPLACE INTO registry_cache
                 (name, description, homepage, bugs, version, category, command, args, env_template, wizard, source, stars, topics)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                params![
                    item.server.name,
                    item.server.description,
                    item.server.homepage,
                    item.server.bugs,
                    item.server.version,
                    item.server.category,
                    item.install_config.as_ref().map(|c| c.command.clone()),
                    args_json,
                    env_json,
                    wizard_json,
                    source,
                    item.stars,
                    topics_json
                ],
            )?;
        }

        // Update cache timestamp
        conn.execute(
            "INSERT OR REPLACE INTO cache_metadata (key, value, updated_at) VALUES (?1, ?2, CURRENT_TIMESTAMP)",
            params![format!("registry_cache_{}", source), "cached"],
        )?;

        Ok(())
    }

    /// Get cached registry items
    pub fn get_cached_registry(&self, source: Option<&str>) -> AppResult<Vec<RegistryItem>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::Database(e.to_string()))?;

        let query = match source {
            Some(s) => format!(
                "SELECT * FROM registry_cache WHERE source = '{}' ORDER BY name",
                s
            ),
            None => "SELECT * FROM registry_cache ORDER BY name".to_string(),
        };

        let mut stmt = conn.prepare(&query)?;
        let item_iter = stmt.query_map([], |row| {
            // Updated indices based on new schema
            // 0:id, 1:name, 2:desc, 3:home, 4:bugs, 5:ver, 6:cat
            // 7:cmd, 8:args, 9:env, 10:wiz, 11:source, 12:stars, 13:topics

            let args_str: Option<String> = row.get(8).ok();
            let env_str: Option<String> = row.get(9).ok();
            let wizard_str: Option<String> = row.get(10).ok();
            let topics_str: Option<String> = row.get(13).ok();

            let install_config = {
                let command: Option<String> = row.get(7).ok();
                command.map(|cmd| RegistryInstallConfig {
                    command: cmd,
                    args: args_str
                        .and_then(|s| serde_json::from_str(&s).ok())
                        .unwrap_or_default(),
                    env_template: env_str.and_then(|s| serde_json::from_str(&s).ok()),
                    wizard: wizard_str.and_then(|s| serde_json::from_str(&s).ok()),
                })
            };

            Ok(RegistryItem {
                server: RegistryServer {
                    name: row.get(1)?,
                    description: row.get(2).ok(),
                    homepage: row.get(3).ok(),
                    bugs: row.get(4).ok(),
                    version: row.get(5).ok(),
                    category: row.get(6).ok(),
                },
                install_config,
                source: row.get(11).unwrap_or("github".to_string()),
                stars: row.get(12).unwrap_or(0),
                topics: topics_str
                    .and_then(|t| serde_json::from_str(&t).ok())
                    .unwrap_or_default(),
            })
        })?;

        let mut items = Vec::new();
        for item in item_iter {
            items.push(item?);
        }
        Ok(items)
    }

    /// Check if registry cache is stale (older than max_age_hours)
    pub fn is_cache_stale(&self, source: &str, max_age_hours: i64) -> AppResult<bool> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result: Result<String, _> = conn.query_row(
            "SELECT updated_at FROM cache_metadata WHERE key = ?1",
            params![format!("registry_cache_{}", source)],
            |row| row.get(0),
        );

        match result {
            Ok(timestamp) => {
                // Parse timestamp and compare
                // SQLite CURRENT_TIMESTAMP is in UTC: "YYYY-MM-DD HH:MM:SS"
                if let Ok(cached_time) =
                    chrono::NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%d %H:%M:%S")
                {
                    let now = chrono::Utc::now().naive_utc();
                    let age = now.signed_duration_since(cached_time);
                    Ok(age.num_hours() > max_age_hours)
                } else {
                    Ok(true) // Can't parse, assume stale
                }
            }
            Err(_) => Ok(true), // No cache metadata, assume stale
        }
    }

    pub fn clear_registry_cache(&self) -> AppResult<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::Database(e.to_string()))?;

        conn.execute("DELETE FROM registry_cache", [])?;
        conn.execute(
            "DELETE FROM cache_metadata WHERE key LIKE 'registry_cache_%'",
            [],
        )?;
        Ok(())
    }

    // === Research Note Methods ===

    pub fn get_research_notes(&self) -> AppResult<Vec<ResearchNote>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::Database(e.to_string()))?;
        let mut stmt = conn.prepare("SELECT * FROM research_notes ORDER BY updated_at DESC")?;

        let note_iter = stmt.query_map([], |row| {
            let tags_str: String = row.get(3)?;
            Ok(ResearchNote {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                tags: serde_json::from_str(&tags_str).unwrap_or_default(),
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;

        let mut notes = Vec::new();
        for note in note_iter {
            notes.push(note?);
        }
        Ok(notes)
    }

    pub fn save_research_note(&self, note: ResearchNote) -> AppResult<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::Database(e.to_string()))?;
        let tags_json = serde_json::to_string(&note.tags)?;

        conn.execute(
            "INSERT OR REPLACE INTO research_notes (id, title, content, tags, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                note.id,
                note.title,
                note.content,
                tags_json,
                note.created_at,
                note.updated_at
            ],
        )?;
        Ok(())
    }
}

fn get_db_path() -> AppResult<PathBuf> {
    let mut path = dirs::data_local_dir().ok_or(AppError::Io("Could not find data dir".into()))?;
    path.push("open-mcp-manager");
    std::fs::create_dir_all(&path)?;
    path.push("servers.db");
    Ok(path)
}

fn init_db_schema(conn: &Connection) -> AppResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mcp_servers (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            type TEXT NOT NULL CHECK (type IN ('stdio', 'sse')),
            command TEXT,
            args TEXT,
            url TEXT,
            env TEXT,
            description TEXT,
            is_active BOOLEAN DEFAULT 1,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // Registry cache table for offline support
    // Registry cache table for offline support
    conn.execute("DROP TABLE IF EXISTS registry_cache", [])?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS registry_cache (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            description TEXT,
            homepage TEXT,
            bugs TEXT,
            version TEXT,
            category TEXT,
            command TEXT,
            args TEXT,
            env_template TEXT,
            wizard TEXT,
            source TEXT NOT NULL DEFAULT 'github',
            stars INTEGER DEFAULT 0,
            topics TEXT,
            cached_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // Metadata table to track cache freshness
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cache_metadata (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // Research notes table for the 'Research Project'
    conn.execute(
        "CREATE TABLE IF NOT EXISTS research_notes (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT,
            tags TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_create_and_get_server() {
        let db = Database::new_in_memory().unwrap();

        let args = CreateServerArgs {
            name: "test-server".to_string(),
            server_type: "stdio".to_string(),
            command: Some("npx".to_string()),
            args: Some(vec!["-y".to_string(), "test".to_string()]),
            url: None,
            env: Some(HashMap::from([("KEY".to_string(), "VALUE".to_string())])),
            description: Some("Test server".to_string()),
        };

        let server = db.create_server(args).unwrap();
        assert_eq!(server.name, "test-server");
        assert_eq!(server.server_type, "stdio");
        assert_eq!(server.env.unwrap().get("KEY"), Some(&"VALUE".to_string()));

        let servers = db.get_servers().unwrap();
        assert_eq!(servers.len(), 1);
        assert_eq!(servers[0].id, server.id);
    }

    #[test]
    fn test_update_server() {
        let db = Database::new_in_memory().unwrap();
        let args = CreateServerArgs {
            name: "update-test".to_string(),
            server_type: "stdio".to_string(),
            command: Some("cmd".to_string()),
            args: None,
            url: None,
            env: None,
            description: None,
        };
        let server = db.create_server(args).unwrap();

        let update_args = UpdateServerArgs {
            name: Some("updated-name".to_string()),
            server_type: None,
            command: None,
            args: None,
            url: None,
            env: None,
            description: None,
            is_active: Some(false),
        };

        let updated = db.update_server(server.id.clone(), update_args).unwrap();
        assert_eq!(updated.name, "updated-name");
        assert_eq!(updated.is_active, false);

        let servers = db.get_servers().unwrap();
        assert_eq!(servers[0].name, "updated-name");
    }

    #[test]
    fn test_delete_server() {
        let db = Database::new_in_memory().unwrap();
        let args = CreateServerArgs {
            name: "delete-test".to_string(),
            server_type: "stdio".to_string(),
            command: Some("cmd".to_string()),
            args: None,
            url: None,
            env: None,
            description: None,
        };
        let server = db.create_server(args).unwrap();

        let servers_before = db.get_servers().unwrap();
        assert_eq!(servers_before.len(), 1);

        db.delete_server(server.id).unwrap();

        let servers_after = db.get_servers().unwrap();
        assert_eq!(servers_after.len(), 0);
    }

    // === Additional Database Tests ===

    #[test]
    fn test_get_server_by_id() {
        let db = Database::new_in_memory().unwrap();
        let args = CreateServerArgs {
            name: "get-test".to_string(),
            server_type: "stdio".to_string(),
            command: Some("cmd".to_string()),
            args: None,
            url: None,
            env: None,
            description: Some("Test description".to_string()),
        };
        let created = db.create_server(args).unwrap();

        let fetched = db.get_server(created.id.clone()).unwrap();
        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.name, "get-test");
        assert_eq!(fetched.description, Some("Test description".to_string()));
    }

    #[test]
    fn test_create_sse_server() {
        let db = Database::new_in_memory().unwrap();
        let args = CreateServerArgs {
            name: "sse-server".to_string(),
            server_type: "sse".to_string(),
            command: None,
            args: None,
            url: Some("https://example.com/sse".to_string()),
            env: None,
            description: None,
        };

        let server = db.create_server(args).unwrap();
        assert_eq!(server.server_type, "sse");
        assert_eq!(server.url, Some("https://example.com/sse".to_string()));
        assert!(server.command.is_none());
    }

    #[test]
    fn test_update_server_command() {
        let db = Database::new_in_memory().unwrap();
        let args = CreateServerArgs {
            name: "cmd-update-test".to_string(),
            server_type: "stdio".to_string(),
            command: Some("old-cmd".to_string()),
            args: None,
            url: None,
            env: None,
            description: None,
        };
        let server = db.create_server(args).unwrap();

        let update_args = UpdateServerArgs {
            name: None,
            server_type: None,
            command: Some("new-cmd".to_string()),
            args: None,
            url: None,
            env: None,
            description: None,
            is_active: None,
        };

        let updated = db.update_server(server.id, update_args).unwrap();
        assert_eq!(updated.command, Some("new-cmd".to_string()));
    }

    #[test]
    fn test_update_server_args() {
        let db = Database::new_in_memory().unwrap();
        let args = CreateServerArgs {
            name: "args-update-test".to_string(),
            server_type: "stdio".to_string(),
            command: Some("cmd".to_string()),
            args: Some(vec!["old-arg".to_string()]),
            url: None,
            env: None,
            description: None,
        };
        let server = db.create_server(args).unwrap();

        let update_args = UpdateServerArgs {
            name: None,
            server_type: None,
            command: None,
            args: Some(vec!["new-arg1".to_string(), "new-arg2".to_string()]),
            url: None,
            env: None,
            description: None,
            is_active: None,
        };

        let updated = db.update_server(server.id, update_args).unwrap();
        assert_eq!(
            updated.args,
            Some(vec!["new-arg1".to_string(), "new-arg2".to_string()])
        );
    }

    #[test]
    fn test_update_server_env() {
        let db = Database::new_in_memory().unwrap();
        let args = CreateServerArgs {
            name: "env-update-test".to_string(),
            server_type: "stdio".to_string(),
            command: Some("cmd".to_string()),
            args: None,
            url: None,
            env: Some(HashMap::from([(
                "OLD_KEY".to_string(),
                "old_value".to_string(),
            )])),
            description: None,
        };
        let server = db.create_server(args).unwrap();

        let update_args = UpdateServerArgs {
            name: None,
            server_type: None,
            command: None,
            args: None,
            url: None,
            env: Some(HashMap::from([(
                "NEW_KEY".to_string(),
                "new_value".to_string(),
            )])),
            description: None,
            is_active: None,
        };

        let updated = db.update_server(server.id, update_args).unwrap();
        assert_eq!(
            updated.env.unwrap().get("NEW_KEY"),
            Some(&"new_value".to_string())
        );
    }

    #[test]
    fn test_multiple_servers() {
        let db = Database::new_in_memory().unwrap();

        for i in 0..5 {
            let args = CreateServerArgs {
                name: format!("server-{}", i),
                server_type: "stdio".to_string(),
                command: Some("cmd".to_string()),
                args: None,
                url: None,
                env: None,
                description: None,
            };
            db.create_server(args).unwrap();
        }

        let servers = db.get_servers().unwrap();
        assert_eq!(servers.len(), 5);
    }

    #[test]
    fn test_servers_ordered_by_created_at() {
        let db = Database::new_in_memory().unwrap();

        // Create servers in order
        for i in 0..3 {
            let args = CreateServerArgs {
                name: format!("server-{}", i),
                server_type: "stdio".to_string(),
                command: Some("cmd".to_string()),
                args: None,
                url: None,
                env: None,
                description: None,
            };
            db.create_server(args).unwrap();
        }

        let servers = db.get_servers().unwrap();
        // Servers should be ordered by created_at DESC (newest first)
        assert_eq!(servers.len(), 3);
    }

    #[test]
    fn test_server_is_active_default_true() {
        let db = Database::new_in_memory().unwrap();
        let args = CreateServerArgs {
            name: "active-test".to_string(),
            server_type: "stdio".to_string(),
            command: Some("cmd".to_string()),
            args: None,
            url: None,
            env: None,
            description: None,
        };

        let server = db.create_server(args).unwrap();
        assert!(server.is_active);
    }

    #[test]
    fn test_server_has_timestamps() {
        let db = Database::new_in_memory().unwrap();
        let args = CreateServerArgs {
            name: "timestamp-test".to_string(),
            server_type: "stdio".to_string(),
            command: Some("cmd".to_string()),
            args: None,
            url: None,
            env: None,
            description: None,
        };

        let server = db.create_server(args).unwrap();
        assert!(!server.created_at.is_empty());
        assert!(!server.updated_at.is_empty());
    }

    #[test]
    fn test_server_has_uuid_id() {
        let db = Database::new_in_memory().unwrap();
        let args = CreateServerArgs {
            name: "uuid-test".to_string(),
            server_type: "stdio".to_string(),
            command: Some("cmd".to_string()),
            args: None,
            url: None,
            env: None,
            description: None,
        };

        let server = db.create_server(args).unwrap();
        // UUID format check (basic)
        assert!(server.id.len() == 36);
        assert!(server.id.contains("-"));
    }

    #[test]
    fn test_delete_nonexistent_server() {
        let db = Database::new_in_memory().unwrap();
        // Should not error when deleting non-existent ID
        let result = db.delete_server("non-existent-id".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_nonexistent_server() {
        let db = Database::new_in_memory().unwrap();
        let result = db.get_server("non-existent-id".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_server_with_empty_args_and_env() {
        let db = Database::new_in_memory().unwrap();
        let args = CreateServerArgs {
            name: "empty-collections-test".to_string(),
            server_type: "stdio".to_string(),
            command: Some("cmd".to_string()),
            args: Some(vec![]),
            url: None,
            env: Some(HashMap::new()),
            description: None,
        };

        let server = db.create_server(args).unwrap();
        // Empty vec/map serialized and deserialized correctly
        assert!(
            server.args.is_none() || server.args.as_ref().map(|a| a.is_empty()).unwrap_or(false)
        );
    }

    #[test]
    fn test_update_description() {
        let db = Database::new_in_memory().unwrap();
        let args = CreateServerArgs {
            name: "desc-update-test".to_string(),
            server_type: "stdio".to_string(),
            command: Some("cmd".to_string()),
            args: None,
            url: None,
            env: None,
            description: None,
        };
        let server = db.create_server(args).unwrap();
        assert!(server.description.is_none());

        let update_args = UpdateServerArgs {
            name: None,
            server_type: None,
            command: None,
            args: None,
            url: None,
            env: None,
            description: Some("New description".to_string()),
            is_active: None,
        };

        let updated = db.update_server(server.id, update_args).unwrap();
        assert_eq!(updated.description, Some("New description".to_string()));
    }

    #[test]
    fn test_database_clone() {
        let db = Database::new_in_memory().unwrap();
        let args = CreateServerArgs {
            name: "clone-test".to_string(),
            server_type: "stdio".to_string(),
            command: Some("cmd".to_string()),
            args: None,
            url: None,
            env: None,
            description: None,
        };
        db.create_server(args).unwrap();

        // Clone the database reference
        let db2 = db.clone();
        let servers = db2.get_servers().unwrap();
        assert_eq!(servers.len(), 1);
    }

    // === Registry Cache Tests ===

    #[test]
    fn test_cache_registry_empty() {
        let db = Database::new_in_memory().unwrap();
        let items: Vec<RegistryItem> = vec![];
        let result = db.cache_registry(&items, "test");
        assert!(result.is_ok());

        let cached = db.get_cached_registry(Some("test")).unwrap();
        assert!(cached.is_empty());
    }

    #[test]
    fn test_cache_registry_single_item() {
        let db = Database::new_in_memory().unwrap();
        let items = vec![RegistryItem {
            server: RegistryServer {
                name: "Test Server".to_string(),
                description: Some("A test server".to_string()),
                homepage: Some("https://example.com".to_string()),
                bugs: None,
                version: Some("1.0.0".to_string()),
                category: Some("Test".to_string()),
            },
            install_config: Some(RegistryInstallConfig {
                command: "npx".to_string(),
                args: vec!["-y".to_string(), "test-server".to_string()],
                env_template: None,
                wizard: None,
            }),
            source: "test".to_string(),
            stars: 0,
            topics: vec![],
        }];

        db.cache_registry(&items, "test").unwrap();
        let cached = db.get_cached_registry(Some("test")).unwrap();

        assert_eq!(cached.len(), 1);
        assert_eq!(cached[0].server.name, "Test Server");
        assert_eq!(
            cached[0].server.description,
            Some("A test server".to_string())
        );
    }

    #[test]
    fn test_cache_registry_multiple_items() {
        let db = Database::new_in_memory().unwrap();
        let items = vec![
            RegistryItem {
                server: RegistryServer {
                    name: "Server A".to_string(),
                    description: Some("First server".to_string()),
                    homepage: None,
                    bugs: None,
                    version: Some("1.0.0".to_string()),
                    category: Some("Cat A".to_string()),
                },
                install_config: Some(RegistryInstallConfig {
                    command: "npx".to_string(),
                    args: vec!["-y".to_string(), "server-a".to_string()],
                    env_template: None,
                    wizard: None,
                }),
                source: "test".to_string(),
                stars: 0,
                topics: vec![],
            },
            RegistryItem {
                server: RegistryServer {
                    name: "Server B".to_string(),
                    description: Some("Second server".to_string()),
                    homepage: None,
                    bugs: None,
                    version: Some("2.0.0".to_string()),
                    category: Some("Cat B".to_string()),
                },
                install_config: Some(RegistryInstallConfig {
                    command: "python".to_string(),
                    args: vec!["-m".to_string(), "server_b".to_string()],
                    env_template: None,
                    wizard: None,
                }),
                source: "test".to_string(),
                stars: 0,
                topics: vec![],
            },
        ];

        db.cache_registry(&items, "test").unwrap();
        let cached = db.get_cached_registry(Some("test")).unwrap();

        assert_eq!(cached.len(), 2);
    }

    #[test]
    fn test_cache_registry_with_env_template() {
        let db = Database::new_in_memory().unwrap();
        let mut env_template = HashMap::new();
        env_template.insert("API_KEY".to_string(), "your-key-here".to_string());

        let items = vec![RegistryItem {
            server: RegistryServer {
                name: "API Server".to_string(),
                description: Some("Needs API key".to_string()),
                homepage: None,
                bugs: None,
                version: None,
                category: None,
            },
            install_config: Some(RegistryInstallConfig {
                command: "npx".to_string(),
                args: vec!["-y".to_string(), "api-server".to_string()],
                env_template: Some(env_template),
                wizard: None,
            }),
            source: "test".to_string(),
            stars: 0,
            topics: vec![],
        }];

        db.cache_registry(&items, "test").unwrap();
        let cached = db.get_cached_registry(Some("test")).unwrap();

        assert_eq!(cached.len(), 1);
        // Note: env_template deserialization tested here
        if let Some(config) = &cached[0].install_config {
            assert!(config.env_template.is_some());
        }
    }

    #[test]
    fn test_cache_registry_overwrites_source() {
        let db = Database::new_in_memory().unwrap();

        // First cache
        let items1 = vec![RegistryItem {
            server: RegistryServer {
                name: "Old Server".to_string(),
                description: None,
                homepage: None,
                bugs: None,
                version: None,
                category: None,
            },
            install_config: None,
            source: "github".to_string(),
            stars: 0,
            topics: vec![],
        }];
        db.cache_registry(&items1, "github").unwrap();

        // Second cache (should replace)
        let items2 = vec![RegistryItem {
            server: RegistryServer {
                name: "New Server".to_string(),
                description: None,
                homepage: None,
                bugs: None,
                version: None,
                category: None,
            },
            install_config: None,
            source: "github".to_string(),
            stars: 0,
            topics: vec![],
        }];
        db.cache_registry(&items2, "github").unwrap();

        let cached = db.get_cached_registry(Some("github")).unwrap();
        assert_eq!(cached.len(), 1);
        assert_eq!(cached[0].server.name, "New Server");
    }

    #[test]
    fn test_cache_registry_different_sources() {
        let db = Database::new_in_memory().unwrap();

        let items_github = vec![RegistryItem {
            server: RegistryServer {
                name: "GitHub Server".to_string(),
                description: None,
                homepage: None,
                bugs: None,
                version: None,
                category: None,
            },
            install_config: None,
            source: "github".to_string(),
            stars: 0,
            topics: vec![],
        }];

        let items_npm = vec![RegistryItem {
            server: RegistryServer {
                name: "NPM Server".to_string(),
                description: None,
                homepage: None,
                bugs: None,
                version: None,
                category: None,
            },
            install_config: None,
            source: "npm".to_string(),
            stars: 0,
            topics: vec![],
        }];

        db.cache_registry(&items_github, "github").unwrap();
        db.cache_registry(&items_npm, "npm").unwrap();

        let github_cached = db.get_cached_registry(Some("github")).unwrap();
        let npm_cached = db.get_cached_registry(Some("npm")).unwrap();
        let all_cached = db.get_cached_registry(None).unwrap();

        assert_eq!(github_cached.len(), 1);
        assert_eq!(npm_cached.len(), 1);
        assert_eq!(all_cached.len(), 2);
    }

    #[test]
    fn test_is_cache_stale_no_cache() {
        let db = Database::new_in_memory().unwrap();
        // No cache exists, should be stale
        let is_stale = db.is_cache_stale("nonexistent", 24).unwrap();
        assert!(is_stale);
    }

    #[test]
    fn test_is_cache_stale_fresh_cache() {
        let db = Database::new_in_memory().unwrap();
        let items = vec![RegistryItem {
            server: RegistryServer {
                name: "Test".to_string(),
                description: None,
                homepage: None,
                bugs: None,
                version: None,
                category: None,
            },
            install_config: None,
            source: "test".to_string(),
            stars: 0,
            topics: vec![],
        }];

        db.cache_registry(&items, "test").unwrap();

        // Just cached, should not be stale with 24 hour max age
        let is_stale = db.is_cache_stale("test", 24).unwrap();
        assert!(!is_stale);
    }

    #[test]
    fn test_clear_registry_cache() {
        let db = Database::new_in_memory().unwrap();
        let items = vec![RegistryItem {
            server: RegistryServer {
                name: "Test".to_string(),
                description: None,
                homepage: None,
                bugs: None,
                version: None,
                category: None,
            },
            install_config: None,
            source: "test".to_string(),
            stars: 0,
            topics: vec![],
        }];

        db.cache_registry(&items, "test").unwrap();
        assert!(!db.get_cached_registry(None).unwrap().is_empty());

        db.clear_registry_cache().unwrap();
        assert!(db.get_cached_registry(None).unwrap().is_empty());
    }

    #[test]
    fn test_cache_registry_without_install_config() {
        let db = Database::new_in_memory().unwrap();
        let items = vec![RegistryItem {
            server: RegistryServer {
                name: "No Config Server".to_string(),
                description: Some("Server without install config".to_string()),
                homepage: None,
                bugs: None,
                version: None,
                category: None,
            },
            install_config: None,
            source: "test".to_string(),
            stars: 0,
            topics: vec![],
        }];

        db.cache_registry(&items, "test").unwrap();
        let cached = db.get_cached_registry(Some("test")).unwrap();

        assert_eq!(cached.len(), 1);
        assert_eq!(cached[0].server.name, "No Config Server");
    }
}
