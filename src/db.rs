use crate::models::{AppError, AppResult, CreateServerArgs, McpServer, UpdateServerArgs};
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
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
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
}
