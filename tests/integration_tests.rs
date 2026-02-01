//! Integration tests for MCP server process management
//!
//! These tests spawn real MCP server processes and verify communication.
//! Tests are marked with `#[ignore]` by default as they require external tools
//! (npx, node, python) to be installed.
//!
//! Run with: cargo test --test integration_tests -- --ignored

use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::timeout;

// We need to import from the main crate
// The McpProcess and ProcessLog are re-exported for testing
use open_mcp_manager::process::{McpProcess, ProcessLog};

/// Helper to create a log channel for tests
fn create_log_channel() -> (mpsc::Sender<ProcessLog>, mpsc::Receiver<ProcessLog>) {
    mpsc::channel::<ProcessLog>(100)
}

fn get_npx_command() -> String {
    if cfg!(windows) {
        "npx.cmd".to_string()
    } else {
        "npx".to_string()
    }
}

/// Test spawning the MCP memory server and performing basic operations
#[tokio::test]
// Requires npx and @modelcontextprotocol/server-memory
async fn test_memory_server_lifecycle() {
    let (log_tx, mut log_rx) = create_log_channel();

    // Start the memory server
    let process = McpProcess::start(
        "test-memory".to_string(),
        get_npx_command(),
        vec![
            "-y".to_string(),
            "@modelcontextprotocol/server-memory".to_string(),
        ],
        None,
        log_tx,
    )
    .await;

    assert!(
        process.is_ok(),
        "Failed to start memory server: {:?}",
        process.err()
    );
    let process = process.unwrap();

    // Give the server time to initialize
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Test: List tools should return available tools
    let tools_result = timeout(Duration::from_secs(15), process.list_tools()).await;
    assert!(tools_result.is_ok(), "List tools timed out");

    if let Ok(Ok(tools)) = tools_result {
        // Memory server should have store_memory and retrieve_memory tools
        println!(
            "Available tools: {:?}",
            tools.iter().map(|t| &t.name).collect::<Vec<_>>()
        );
        assert!(!tools.is_empty(), "Memory server should have tools");
    }

    // Clean up: Kill the server
    let kill_result = process.kill().await;
    assert!(kill_result.is_ok(), "Failed to kill server");

    // Drain any remaining logs
    while log_rx.try_recv().is_ok() {}
}

/// Test spawning the echo server (if available) for basic JSON-RPC communication
#[tokio::test]
// Requires npx and @modelcontextprotocol/server-everything
async fn test_everything_server_tools_list() {
    let (log_tx, _log_rx) = create_log_channel();

    let process = McpProcess::start(
        "test-everything".to_string(),
        get_npx_command(),
        vec![
            "-y".to_string(),
            "@modelcontextprotocol/server-everything".to_string(),
        ],
        None,
        log_tx,
    )
    .await;

    if let Err(e) = &process {
        println!("Skipping test: Could not start everything server: {}", e);
        return;
    }

    let process = process.unwrap();
    tokio::time::sleep(Duration::from_secs(3)).await;

    // List tools
    let tools_result = timeout(Duration::from_secs(10), process.list_tools()).await;
    if let Ok(Ok(tools)) = tools_result {
        println!("Everything server tools: {:?}", tools.len());
        assert!(!tools.is_empty());
    }

    // List resources
    let resources_result = timeout(Duration::from_secs(10), process.list_resources()).await;
    if let Ok(Ok(resources)) = resources_result {
        println!("Everything server resources: {:?}", resources.len());
    }

    // List prompts
    let prompts_result = timeout(Duration::from_secs(10), process.list_prompts()).await;
    if let Ok(Ok(prompts)) = prompts_result {
        println!("Everything server prompts: {:?}", prompts.len());
    }

    let _ = process.kill().await;
}

/// Test that the process correctly handles server startup failure
#[tokio::test]
async fn test_invalid_command_fails() {
    let (log_tx, _log_rx) = create_log_channel();

    let result = McpProcess::start(
        "test-invalid".to_string(),
        "nonexistent-command-that-does-not-exist-12345".to_string(),
        vec![],
        None,
        log_tx,
    )
    .await;

    assert!(result.is_err(), "Should fail with invalid command");
}

/// Test environment variable passing to MCP server
#[tokio::test]
// Requires node to be installed
async fn test_env_vars_passed_to_process() {
    let (log_tx, mut log_rx) = create_log_channel();

    // Create a simple node script that prints an env var
    let script = r#"console.log('ENV_TEST=' + process.env.TEST_VAR)"#;

    let mut env = std::collections::HashMap::new();
    env.insert("TEST_VAR".to_string(), "hello_from_test".to_string());

    let result = McpProcess::start(
        "test-env".to_string(),
        "node".to_string(),
        vec!["-e".to_string(), script.to_string()],
        Some(env),
        log_tx,
    )
    .await;

    if result.is_err() {
        println!("Skipping test: Node not available");
        return;
    }

    let process = result.unwrap();

    // Wait for output
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Check if we received the env var in stdout
    let mut found = false;
    while let Ok(log) = log_rx.try_recv() {
        if let ProcessLog::Stdout(msg) = log {
            if msg.contains("ENV_TEST=hello_from_test") {
                found = true;
                break;
            }
        }
    }

    let _ = process.kill().await;
    assert!(found, "Should have received env var output");
}

/// Test multiple concurrent server instances
#[tokio::test]
// Requires npx
async fn test_multiple_concurrent_servers() {
    let (log_tx1, _) = create_log_channel();
    let (log_tx2, _) = create_log_channel();

    // Start two memory servers concurrently
    let result1 = McpProcess::start(
        "test-multi-1".to_string(),
        get_npx_command(),
        vec![
            "-y".to_string(),
            "@modelcontextprotocol/server-memory".to_string(),
        ],
        None,
        log_tx1,
    )
    .await;

    let result2 = McpProcess::start(
        "test-multi-2".to_string(),
        get_npx_command(),
        vec![
            "-y".to_string(),
            "@modelcontextprotocol/server-memory".to_string(),
        ],
        None,
        log_tx2,
    )
    .await;

    // Both should start successfully
    if result1.is_err() || result2.is_err() {
        println!("Skipping test: Could not start servers");
        return;
    }

    let process1 = result1.unwrap();
    let process2 = result2.unwrap();

    // Give servers time to start
    tokio::time::sleep(Duration::from_secs(3)).await;

    // Both should respond to list_tools
    let tools1 = timeout(Duration::from_secs(15), process1.list_tools()).await;
    let tools2 = timeout(Duration::from_secs(15), process2.list_tools()).await;

    assert!(tools1.is_ok(), "Server 1 should respond");
    assert!(tools2.is_ok(), "Server 2 should respond");

    // Clean up
    let _ = process1.kill().await;
    let _ = process2.kill().await;
}

/// Test that killing a process works correctly
#[tokio::test]
// Requires node
async fn test_process_kill() {
    let (log_tx, _) = create_log_channel();

    // Start a long-running node process
    let script = r#"setInterval(() => {}, 1000)"#;

    let result = McpProcess::start(
        "test-kill".to_string(),
        "node".to_string(),
        vec!["-e".to_string(), script.to_string()],
        None,
        log_tx,
    )
    .await;

    if result.is_err() {
        println!("Skipping test: Node not available");
        return;
    }

    let process = result.unwrap();

    // Give it time to start
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Kill should succeed
    let kill_result = process.kill().await;
    assert!(kill_result.is_ok(), "Kill should succeed");
}

/// Test request ID incrementing
#[tokio::test]
// Requires npx and @modelcontextprotocol/server-memory
async fn test_request_id_increments() {
    let (log_tx, _) = create_log_channel();

    let result = McpProcess::start(
        "test-ids".to_string(),
        get_npx_command(),
        vec![
            "-y".to_string(),
            "@modelcontextprotocol/server-memory".to_string(),
        ],
        None,
        log_tx,
    )
    .await;

    if result.is_err() {
        println!("Skipping test: Memory server not available");
        return;
    }

    let process = result.unwrap();
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Make multiple requests
    let _ = timeout(Duration::from_secs(5), process.list_tools()).await;
    let _ = timeout(Duration::from_secs(5), process.list_resources()).await;
    let _ = timeout(Duration::from_secs(5), process.list_prompts()).await;

    // Check that request ID has incremented
    let id = *process.next_request_id.lock().await;
    assert!(id > 3, "Request ID should have incremented: {}", id);

    let _ = process.kill().await;
}

/// Test stderr logging
#[tokio::test]
// Requires node
async fn test_stderr_logging() {
    let (log_tx, mut log_rx) = create_log_channel();

    // Script that writes to stderr
    let script = r#"console.error('This is an error message')"#;

    let result = McpProcess::start(
        "test-stderr".to_string(),
        "node".to_string(),
        vec!["-e".to_string(), script.to_string()],
        None,
        log_tx,
    )
    .await;

    if result.is_err() {
        println!("Skipping test: Node not available");
        return;
    }

    let process = result.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Check for stderr output
    let mut found_stderr = false;
    while let Ok(log) = log_rx.try_recv() {
        if let ProcessLog::Stderr(msg) = log {
            if msg.contains("error message") {
                found_stderr = true;
                break;
            }
        }
    }

    let _ = process.kill().await;
    assert!(found_stderr, "Should have captured stderr output");
}
