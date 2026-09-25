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
            eprintln!(
                "agent adapter '{command}' is not yet available; use sanctum-serve and the printed API endpoint"
            );
            serve().await?;
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
