mod models;
mod endpoints;
mod utils;
mod templates;
mod server;

use utils::Logger;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

fn is_stdio_mode() -> bool {
    // #region agent log
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/Users/huffmullen/mcp/flakes/.cursor/debug.log") {
        let _ = writeln!(f, r#"{{"id":"log_main_001","timestamp":{},"location":"main.rs:10","message":"is_stdio_mode check","data":{{"port_env":{:?},"is_tty":{}}},"sessionId":"debug-session","runId":"run1","hypothesisId":"A"}}"#, 
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            std::env::var("PORT"),
            atty::is(atty::Stream::Stdin)
        );
    }
    // #endregion
    // Check if we're running in stdio mode (MCP) vs HTTP mode
    // MCP servers should always use stdio when stdin is not a TTY
    !atty::is(atty::Stream::Stdin)
}

#[tokio::main]
async fn main() {
    // #region agent log - EARLY LOGGING
    use std::io::Write;
    let _ = std::fs::create_dir_all("/Users/huffmullen/mcp/flakes/.cursor");
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/Users/huffmullen/mcp/flakes/.cursor/debug.log") {
        let _ = writeln!(f, r#"{{"id":"log_main_early","timestamp":{},"location":"main.rs:17","message":"MAIN STARTED","data":{{"pid":{}}},"sessionId":"debug-session","runId":"run2","hypothesisId":"A"}}"#, 
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            std::process::id()
        );
    }
    // #endregion
    // #region agent log
    let stdio_mode = is_stdio_mode();
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/Users/huffmullen/mcp/flakes/.cursor/debug.log") {
        let _ = writeln!(f, r#"{{"id":"log_main_002","timestamp":{},"location":"main.rs:17","message":"main entry","data":{{"stdio_mode":{},"port_env":{:?},"is_tty_stdin":{},"is_tty_stdout":{},"is_tty_stderr":{}}},"sessionId":"debug-session","runId":"run2","hypothesisId":"A"}}"#, 
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            stdio_mode,
            std::env::var("PORT"),
            atty::is(atty::Stream::Stdin),
            atty::is(atty::Stream::Stdout),
            atty::is(atty::Stream::Stderr)
        );
    }
    // #endregion
    if stdio_mode {
        // #region agent log
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/Users/huffmullen/mcp/flakes/.cursor/debug.log") {
            let _ = writeln!(f, r#"{{"id":"log_main_003","timestamp":{},"location":"main.rs:19","message":"entering stdio mode branch","data":{{}},"sessionId":"debug-session","runId":"run1","hypothesisId":"D"}}"#, 
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()
            );
        }
        // #endregion
        // MCP stdio mode
        Logger::init_stdio();
        Logger::info("Starting Nix Flakes MCP Server (rust-2.0) - stdio mode");
        
        let stdin = tokio::io::stdin();
        let mut stdin = BufReader::new(stdin);
        let mut stdout = tokio::io::stdout();
        let mut line = String::new();

        loop {
            line.clear();
            match stdin.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    
                    match server::handle_mcp_stdio_request(trimmed).await {
                        Ok(Some(response)) => {
                            let json = serde_json::to_string(&response).unwrap();
                            // #region agent log
                            use std::io::Write;
                            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/Users/huffmullen/mcp/flakes/.cursor/debug.log") {
                                let _ = writeln!(f, r#"{{"id":"log_main_005","timestamp":{},"location":"main.rs:82","message":"WRITING RESPONSE TO STDOUT","data":{{"json_len":{},"json_preview":{:?}}},"sessionId":"debug-session","runId":"run3","hypothesisId":"A"}}"#,
                                    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
                                    json.len(),
                                    if json.len() > 200 { format!("{}...", &json[..200]) } else { json.clone() }
                                );
                            }
                            // #endregion
                            stdout.write_all(json.as_bytes()).await.unwrap();
                            stdout.write_all(b"\n").await.unwrap();
                            stdout.flush().await.unwrap();
                        }
                        Ok(None) => {
                            // Notification, no response needed
                        }
                        Err(e) => {
                            let error_response = serde_json::json!({
                                "jsonrpc": "2.0",
                                "error": {
                                    "code": -32603,
                                    "message": format!("Internal error: {}", e)
                                },
                                "id": null
                            });
                            let json = serde_json::to_string(&error_response).unwrap();
                            stdout.write_all(json.as_bytes()).await.unwrap();
                            stdout.write_all(b"\n").await.unwrap();
                            stdout.flush().await.unwrap();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from stdin: {}", e);
                    break;
                }
            }
        }
    } else {
        // #region agent log
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/Users/huffmullen/mcp/flakes/.cursor/debug.log") {
            let _ = writeln!(f, r#"{{"id":"log_main_004","timestamp":{},"location":"main.rs:70","message":"entering HTTP mode branch - ERROR - THIS SHOULD NOT HAPPEN FOR MCP","data":{{"port_env":{:?},"is_tty_stdin":{}}},"sessionId":"debug-session","runId":"run2","hypothesisId":"D"}}"#, 
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
                std::env::var("PORT"),
                atty::is(atty::Stream::Stdin)
            );
        }
        // #endregion
        // HTTP mode - FORCE STDIO MODE FOR MCP
        // MCP servers MUST use stdio, so if we somehow got here, we should still use stdio
        eprintln!("WARNING: Detected TTY stdin but forcing stdio mode for MCP compatibility");
        Logger::init_stdio();
        Logger::info("Starting Nix Flakes MCP Server (rust-2.0) - stdio mode (forced)");
        
        let stdin = tokio::io::stdin();
        let mut stdin = BufReader::new(stdin);
        let mut stdout = tokio::io::stdout();
        let mut line = String::new();

        loop {
            line.clear();
            match stdin.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    
                    match server::handle_mcp_stdio_request(trimmed).await {
                        Ok(Some(response)) => {
                            let json = serde_json::to_string(&response).unwrap();
                            // #region agent log
                            use std::io::Write;
                            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("/Users/huffmullen/mcp/flakes/.cursor/debug.log") {
                                let _ = writeln!(f, r#"{{"id":"log_main_005","timestamp":{},"location":"main.rs:82","message":"WRITING RESPONSE TO STDOUT","data":{{"json_len":{},"json_preview":{:?}}},"sessionId":"debug-session","runId":"run3","hypothesisId":"A"}}"#,
                                    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
                                    json.len(),
                                    if json.len() > 200 { format!("{}...", &json[..200]) } else { json.clone() }
                                );
                            }
                            // #endregion
                            stdout.write_all(json.as_bytes()).await.unwrap();
                            stdout.write_all(b"\n").await.unwrap();
                            stdout.flush().await.unwrap();
                        }
                        Ok(None) => {
                            // Notification, no response needed
                        }
                        Err(e) => {
                            let error_response = serde_json::json!({
                                "jsonrpc": "2.0",
                                "error": {
                                    "code": -32603,
                                    "message": format!("Internal error: {}", e)
                                },
                                "id": null
                            });
                            let json = serde_json::to_string(&error_response).unwrap();
                            stdout.write_all(json.as_bytes()).await.unwrap();
                            stdout.write_all(b"\n").await.unwrap();
                            stdout.flush().await.unwrap();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from stdin: {}", e);
                    break;
                }
            }
        }
        return;
        // OLD HTTP CODE BELOW (commented out to prevent execution)
        /*
        Logger::init();
        Logger::info("Starting Nix Flakes MCP Server (rust-2.0) - HTTP mode");

        let routes = server::create_routes();

        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);

        Logger::info(&format!("Server listening on port {}", port));

        warp::serve(routes)
            .run(([0, 0, 0, 0], port))
            .await;
        */

        let routes = server::create_routes();

        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);

        Logger::info(&format!("Server listening on port {}", port));

        warp::serve(routes)
            .run(([0, 0, 0, 0], port))
            .await;
    }
}
