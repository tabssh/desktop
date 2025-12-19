//! SSH connection test - validates SSH functionality without GUI

use std::io::{self, Write};
use std::sync::Arc;
use tokio::runtime::Runtime;
use anyhow::Result;

use russh::client;
use russh::keys::key;
use russh::{ChannelMsg, Disconnect};

struct TestHandler {
    #[allow(dead_code)]
    host: String,
}

impl TestHandler {
    fn new(host: &str) -> Self {
        Self { host: host.to_string() }
    }
}

#[async_trait::async_trait]
impl client::Handler for TestHandler {
    type Error = anyhow::Error;

    async fn check_server_key(
        &mut self,
        server_public_key: &key::PublicKey,
    ) -> Result<bool, Self::Error> {
        println!("Server key fingerprint: {}", server_public_key.fingerprint());
        Ok(true)
    }
}

async fn run_ssh_test_password(host: &str, port: u16, username: &str, password: &str) -> Result<()> {
    println!("\n=== TabSSH Connection Test (Password Auth) ===\n");
    println!("Connecting to {}@{}:{}...", username, host, port);

    let config = client::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(60)),
        ..Default::default()
    };

    let addr = format!("{}:{}", host, port);
    let handler = TestHandler::new(host);

    let mut handle = client::connect(Arc::new(config), &addr, handler).await?;
    println!("Connected! Authenticating with password...");

    let authenticated = handle.authenticate_password(username, password).await?;
    if !authenticated {
        return Err(anyhow::anyhow!("Password authentication failed"));
    }
    println!("Authentication successful!\n");

    run_shell_test(handle).await
}

async fn run_ssh_test_key(host: &str, port: u16, username: &str, key_path: &str) -> Result<()> {
    println!("\n=== TabSSH Connection Test (Key Auth) ===\n");
    println!("Connecting to {}@{}:{}...", username, host, port);
    println!("Using key: {}", key_path);

    let config = client::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(60)),
        ..Default::default()
    };

    let addr = format!("{}:{}", host, port);
    let handler = TestHandler::new(host);

    let mut handle = client::connect(Arc::new(config), &addr, handler).await?;
    println!("Connected! Authenticating with key...");

    let key_data = tokio::fs::read_to_string(key_path).await?;
    let key_pair = russh_keys::decode_secret_key(&key_data, None)?;

    let authenticated = handle.authenticate_publickey(username, Arc::new(key_pair)).await?;
    if !authenticated {
        return Err(anyhow::anyhow!("Key authentication failed"));
    }
    println!("Authentication successful!\n");

    run_shell_test(handle).await
}

async fn run_shell_test(handle: client::Handle<TestHandler>) -> Result<()> {

    println!("Opening shell channel...");
    let mut channel = handle.channel_open_session().await?;

    println!("Requesting PTY...");
    channel.request_pty(false, "xterm-256color", 80, 24, 0, 0, &[]).await?;

    println!("Starting shell...");
    channel.request_shell(false).await?;

    println!("\n--- Shell output (first 5 seconds) ---\n");

    let timeout = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        async {
            loop {
                match channel.wait().await {
                    Some(ChannelMsg::Data { data }) => {
                        let text = String::from_utf8_lossy(&data);
                        print!("{}", text);
                        io::stdout().flush().ok();
                    }
                    Some(ChannelMsg::Eof) => {
                        println!("\n[EOF received]");
                        break;
                    }
                    Some(ChannelMsg::Close) => {
                        println!("\n[Channel closed]");
                        break;
                    }
                    Some(msg) => {
                        println!("[Other message: {:?}]", msg);
                    }
                    None => {
                        println!("\n[Channel ended]");
                        break;
                    }
                }
            }
        }
    ).await;

    if timeout.is_err() {
        println!("\n[Timeout - connection working!]");
    }

    println!("\n--- End of shell output ---\n");

    println!("Closing connection...");
    handle.disconnect(Disconnect::ByApplication, "Test complete", "en").await?;

    println!("\n=== Test Passed! ===\n");
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("TabSSH SSH Connection Tester\n");
        eprintln!("Usage:");
        eprintln!("  Password auth: {} -p <host> <username> <password> [port]", args[0]);
        eprintln!("  Key auth:      {} -k <host> <username> <key_path> [port]", args[0]);
        eprintln!("\nExamples:");
        eprintln!("  {} -p 192.168.1.1 root mypassword", args[0]);
        eprintln!("  {} -k localhost root ~/.ssh/id_rsa", args[0]);
        eprintln!("  {} -k example.com admin /root/.ssh/id_ed25519 2222", args[0]);
        std::process::exit(1);
    }

    let mode = &args[1];

    if args.len() < 5 {
        eprintln!("Error: Not enough arguments. Run without arguments for usage.");
        std::process::exit(1);
    }

    let host = &args[2];
    let username = &args[3];
    let credential = &args[4];
    let port: u16 = args.get(5).and_then(|p| p.parse().ok()).unwrap_or(22);

    let runtime = Runtime::new().expect("Failed to create runtime");

    let result = match mode.as_str() {
        "-p" | "--password" => {
            runtime.block_on(run_ssh_test_password(host, port, username, credential))
        }
        "-k" | "--key" => {
            let key_path = shellexpand::tilde(credential);
            runtime.block_on(run_ssh_test_key(host, port, username, &key_path))
        }
        _ => {
            eprintln!("Unknown mode: {}. Use -p for password or -k for key auth.", mode);
            std::process::exit(1);
        }
    };

    match result {
        Ok(_) => {
            println!("SSH connection test completed successfully!");
        }
        Err(e) => {
            eprintln!("\nError: {}", e);
            std::process::exit(1);
        }
    }
}
