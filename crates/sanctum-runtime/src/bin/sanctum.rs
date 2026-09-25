use sanctum_runtime::{api, benchmark};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executable = std::env::args()
        .next()
        .and_then(|path| {
            std::path::PathBuf::from(path)
                .file_stem()
                .map(std::ffi::OsStr::to_owned)
        })
        .and_then(|name| name.into_string().ok())
        .unwrap_or_else(|| "sanctum".to_owned());
    let mut args = std::env::args().skip(1);
    let command = if executable.starts_with("sanctum-") {
        executable.clone()
    } else {
        args.next().unwrap_or_else(|| "serve".to_owned())
    };
    match command.as_str() {
        "serve" | "sanctum-serve" => serve().await?,
        "doctor" | "sanctum-doctor" => doctor()?,
        "benchmark" | "sanctum-benchmark" => benchmark()?,
        "claude" | "codex" | "opencode" | "aider" | "sanctum-claude" | "sanctum-codex"
        | "sanctum-opencode" | "sanctum-aider" => {
            run_agent(&command).await?;
        }
        "--help" | "help" => help(),
        other => {
            eprintln!("unknown command: {other}");
            help();
            std::process::exit(2);
        }
    }
    Ok(())
}

async fn run_agent(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let host = std::env::var("SANCTUM_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = std::env::var("SANCTUM_PORT").unwrap_or_else(|_| "8080".to_owned());
    let endpoint = format!("http://{host}:{port}");
    let client = reqwest::Client::new();
    let server = if client
        .get(format!("{endpoint}/health"))
        .send()
        .await
        .is_ok()
    {
        None
    } else {
        let child = std::process::Command::new(std::env::current_exe()?)
            .arg("serve")
            .spawn()?;
        let mut ready = false;
        for _ in 0..50 {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            if client
                .get(format!("{endpoint}/health"))
                .send()
                .await
                .is_ok()
            {
                ready = true;
                break;
            }
        }
        if !ready {
            return Err("sanctum server did not become ready".into());
        }
        Some(child)
    };
    let (program, variables): (String, Vec<(String, String)>) = match command {
        "claude" | "sanctum-claude" => (
            "claude".into(),
            vec![
                ("ANTHROPIC_BASE_URL".into(), endpoint.clone()),
                ("ANTHROPIC_API_KEY".into(), "local".into()),
                ("ANTHROPIC_AUTH_TOKEN".into(), "local".into()),
            ],
        ),
        "codex" | "sanctum-codex" | "opencode" | "sanctum-opencode" => (
            command.trim_start_matches("sanctum-").to_owned(),
            vec![
                ("OPENAI_BASE_URL".into(), format!("{endpoint}/v1")),
                ("OPENAI_API_KEY".into(), "local".into()),
            ],
        ),
        "aider" | "sanctum-aider" => (
            "aider".into(),
            vec![
                ("OPENAI_API_BASE".into(), format!("{endpoint}/v1")),
                ("OPENAI_API_KEY".into(), "local".into()),
            ],
        ),
        _ => return Err(format!("unknown agent adapter: {command}").into()),
    };
    let status = std::process::Command::new(&program)
        .envs(variables)
        .args(std::env::args().skip(1))
        .status()?;
    if let Some(mut child) = server {
        let _ = child.kill();
        let _ = child.wait();
    }
    if !status.success() {
        return Err(format!("{program} exited with {status}").into());
    }
    Ok(())
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let host = std::env::var("SANCTUM_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = std::env::var("SANCTUM_PORT").unwrap_or_else(|_| "8080".to_owned());
    let upstream =
        std::env::var("SANCTUM_UPSTREAM").unwrap_or_else(|_| "http://127.0.0.1:11434".to_owned());
    let model = std::env::var("SANCTUM_MODEL").unwrap_or_else(|_| "qwen3.5:4b-q4_K_M".to_owned());
    let address = format!("{host}:{port}");
    let listener = tokio::net::TcpListener::bind(&address).await?;
    println!("Silicon Sanctum listening on http://{address}");
    println!("OpenAI API: http://{address}/v1");
    println!("Anthropic API: http://{address}/v1/messages");
    println!("Upstream: {upstream} ({model})");
    axum::serve(listener, api::router(api::ApiState::new(upstream, model))).await?;
    Ok(())
}

fn doctor() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(&benchmark::probe())?);
    Ok(())
}

fn benchmark() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(&benchmark::probe())?);
    Ok(())
}

fn help() {
    println!("sanctum-serve | sanctum-doctor | sanctum-benchmark");
    println!("Environment: SANCTUM_HOST, SANCTUM_PORT, SANCTUM_UPSTREAM, SANCTUM_MODEL");
}
