//! Small, repeatable host probe used by installation and doctor commands.
#![allow(clippy::missing_errors_doc)]

use serde::Serialize;
use std::{fs, time::Instant};

#[derive(Debug, Clone, Serialize)]
pub struct HardwareReport {
    pub platform: String,
    pub logical_cpus: usize,
    pub memory_bytes: Option<u64>,
    pub storage_probe_ms: u128,
    pub recommendation: &'static str,
    pub comfortable_models: Vec<&'static str>,
    pub robust_but_delayed_models: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BackendReport {
    pub upstream: String,
    pub reachable: bool,
    pub model: String,
    pub model_available: bool,
    pub generation_ms: Option<u128>,
    pub output_tokens: Option<u64>,
    pub tokens_per_second: Option<f64>,
    pub error: Option<String>,
}

#[allow(clippy::cast_precision_loss)]
#[allow(clippy::too_many_lines)]
pub async fn backend_report(upstream: &str, model: &str, run_generation: bool) -> BackendReport {
    let client = reqwest::Client::new();
    let version = client
        .get(format!("{}/api/version", upstream.trim_end_matches('/')))
        .send()
        .await;
    if let Err(error) = version {
        return BackendReport {
            upstream: upstream.to_owned(),
            reachable: false,
            model: model.to_owned(),
            model_available: false,
            generation_ms: None,
            output_tokens: None,
            tokens_per_second: None,
            error: Some(error.to_string()),
        };
    }
    let tags = client
        .get(format!("{}/api/tags", upstream.trim_end_matches('/')))
        .send()
        .await;
    let model_available = match tags {
        Ok(response) => response
            .json::<serde_json::Value>()
            .await
            .ok()
            .and_then(|value| value.get("models").cloned())
            .and_then(|models| models.as_array().cloned())
            .is_some_and(|models| {
                models.iter().any(|entry| {
                    entry.get("name").and_then(serde_json::Value::as_str) == Some(model)
                })
            }),
        Err(_) => false,
    };
    if !model_available {
        return BackendReport {
            upstream: upstream.to_owned(),
            reachable: true,
            model: model.to_owned(),
            model_available: false,
            generation_ms: None,
            output_tokens: None,
            tokens_per_second: None,
            error: Some("configured model is not present in the backend".to_owned()),
        };
    }
    if !run_generation {
        return BackendReport {
            upstream: upstream.to_owned(),
            reachable: true,
            model: model.to_owned(),
            model_available: true,
            generation_ms: None,
            output_tokens: None,
            tokens_per_second: None,
            error: None,
        };
    }
    let started = Instant::now();
    let response = client
        .post(format!("{}/api/generate", upstream.trim_end_matches('/')))
        .json(&serde_json::json!({"model":model,"prompt":"Reply with exactly OK","stream":false,"options":{"num_predict":8}}))
        .timeout(std::time::Duration::from_secs(120))
        .send()
        .await;
    match response {
        Ok(response) => match response.json::<serde_json::Value>().await {
            Ok(value) => {
                let generation_ms = started.elapsed().as_millis();
                let output_tokens = value.get("eval_count").and_then(serde_json::Value::as_u64);
                let tokens_per_second = output_tokens.and_then(|tokens| {
                    (generation_ms > 0).then_some(tokens as f64 / (generation_ms as f64 / 1000.0))
                });
                BackendReport {
                    upstream: upstream.to_owned(),
                    reachable: true,
                    model: model.to_owned(),
                    model_available: true,
                    generation_ms: Some(generation_ms),
                    output_tokens,
                    tokens_per_second,
                    error: None,
                }
            }
            Err(error) => BackendReport {
                upstream: upstream.to_owned(),
                reachable: true,
                model: model.to_owned(),
                model_available: true,
                generation_ms: None,
                output_tokens: None,
                tokens_per_second: None,
                error: Some(error.to_string()),
            },
        },
        Err(error) => BackendReport {
            upstream: upstream.to_owned(),
            reachable: true,
            model: model.to_owned(),
            model_available: true,
            generation_ms: None,
            output_tokens: None,
            tokens_per_second: None,
            error: Some(error.to_string()),
        },
    }
}

#[must_use]
pub fn probe() -> HardwareReport {
    let logical_cpus = std::thread::available_parallelism().map_or(1, usize::from);
    let memory_bytes = memory_bytes();
    let started = Instant::now();
    let _ = fs::read_dir(".").map(std::iter::Iterator::count);
    let storage_probe_ms = started.elapsed().as_millis();
    let (recommendation, comfortable_models, robust_but_delayed_models) = match memory_bytes {
        Some(bytes) if bytes >= 64 * 1024 * 1024 * 1024 => (
            "quality-and-long-context",
            vec!["Qwen3.5 4B", "Qwen3.5 9B", "Qwen3.8 27B"],
            vec!["verified 1M-context runtime"],
        ),
        Some(bytes) if bytes >= 32 * 1024 * 1024 * 1024 => (
            "quality-and-mid-size-models",
            vec!["Qwen3.5 4B", "Qwen3.5 9B"],
            vec!["Qwen3.8 27B", "long-context profiles"],
        ),
        Some(bytes) if bytes >= 16 * 1024 * 1024 * 1024 => (
            "small-and-mid-size-models",
            vec!["Qwen3.5 4B"],
            vec!["Qwen3.5 9B", "Qwen3.8 27B"],
        ),
        Some(_) => ("small-models-only", vec!["Qwen3.5 4B"], vec![]),
        None => ("run-backend-benchmark", vec![], vec![]),
    };
    HardwareReport {
        platform: std::env::consts::OS.to_owned(),
        logical_cpus,
        memory_bytes,
        storage_probe_ms,
        recommendation,
        comfortable_models,
        robust_but_delayed_models,
    }
}

fn memory_bytes() -> Option<u64> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("sysctl")
            .args(["-n", "hw.memsize"])
            .output()
            .ok()?;
        return String::from_utf8(output.stdout).ok()?.trim().parse().ok();
    }
    #[cfg(target_os = "linux")]
    {
        let text = fs::read_to_string("/proc/meminfo").ok()?;
        let kib = text
            .lines()
            .find(|line| line.starts_with("MemTotal:"))?
            .split_whitespace()
            .nth(1)?
            .parse::<u64>()
            .ok()?;
        return Some(kib.saturating_mul(1024));
    }
    #[allow(unreachable_code)]
    None
}
