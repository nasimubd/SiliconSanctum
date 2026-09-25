//! `OpenAI` and Anthropic compatible local gateway.
#![allow(clippy::missing_errors_doc)]

use axum::{
    Json, Router,
    body::Body,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use futures_util::StreamExt;
use reqwest::Client;
use serde_json::{Value, json};
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiState {
    pub client: Client,
    pub upstream: String,
    pub default_model: String,
}

impl ApiState {
    #[must_use]
    pub fn new(upstream: impl Into<String>, default_model: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            upstream: upstream.into().trim_end_matches('/').to_owned(),
            default_model: default_model.into(),
        }
    }
}

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/models", get(models))
        .route("/v1/chat/completions", post(openai_chat))
        .route("/v1/responses", post(openai_chat))
        .route("/v1/completions", post(openai_chat))
        .route("/v1/messages", post(anthropic_messages))
        .with_state(Arc::new(state))
}

async fn health(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    let upstream = state
        .client
        .get(format!("{}/api/version", state.upstream))
        .send()
        .await;
    match upstream {
        Ok(response) if response.status().is_success() => (
            StatusCode::OK,
            Json(json!({
                "status": "ok",
                "upstream": "ready",
                "model": state.default_model,
            })),
        ),
        _ => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "status": "degraded",
                "upstream": "unavailable",
                "model": state.default_model,
            })),
        ),
    }
}

async fn models(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    let response = state
        .client
        .get(format!("{}/api/tags", state.upstream))
        .send()
        .await;
    match response {
        Ok(response) => match response.json::<Value>().await {
            Ok(value) => {
                let models = value
                    .get("models")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|model| {
                        json!({
                            "id": model.get("name").and_then(Value::as_str).unwrap_or(&state.default_model),
                            "object": "model",
                            "owned_by": "local",
                        })
                    })
                    .collect::<Vec<_>>();
                (
                    StatusCode::OK,
                    Json(json!({"object":"list", "data":models})),
                )
            }
            Err(error) => upstream_error(error.to_string()),
        },
        Err(error) => upstream_error(error.to_string()),
    }
}

async fn openai_chat(
    State(state): State<Arc<ApiState>>,
    uri: axum::http::Uri,
    Json(request): Json<Value>,
) -> Response {
    let model = request
        .get("model")
        .and_then(Value::as_str)
        .unwrap_or(&state.default_model);
    let prompt = openai_prompt(&request);
    let ollama = json!({
        "model": model,
        "messages": [{"role":"user", "content": prompt}],
        "stream": request.get("stream").and_then(Value::as_bool).unwrap_or(false),
        "options": {"num_ctx": request.get("max_tokens").and_then(Value::as_u64).unwrap_or(32768)}
    });
    let response = state
        .client
        .post(format!("{}/api/chat", state.upstream))
        .json(&ollama)
        .send()
        .await;
    match response {
        Ok(response)
            if request
                .get("stream")
                .and_then(Value::as_bool)
                .unwrap_or(false) =>
        {
            openai_stream(response, model.to_owned()).into_response()
        }
        Ok(response) => match response.json::<Value>().await {
            Ok(value) => {
                let content = value
                    .pointer("/message/content")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                let response = if uri.path().ends_with("/responses") {
                    json!({
                        "id":"resp_sanctum_local",
                        "object":"response",
                        "status":"completed",
                        "model":model,
                        "output":[{"type":"message","role":"assistant","content":[{"type":"output_text","text":content}]}],
                        "output_text":content,
                        "usage":{"input_tokens":value.get("prompt_eval_count").and_then(Value::as_u64).unwrap_or(0),"output_tokens":value.get("eval_count").and_then(Value::as_u64).unwrap_or(0)}
                    })
                } else {
                    json!({
                        "id":"sanctum-local",
                        "object":"chat.completion",
                        "model":model,
                        "choices":[{"index":0,"message":{"role":"assistant","content":content},"finish_reason":"stop"}],
                        "usage":{"prompt_tokens":value.get("prompt_eval_count").and_then(Value::as_u64).unwrap_or(0),"completion_tokens":value.get("eval_count").and_then(Value::as_u64).unwrap_or(0)}
                    })
                };
                (StatusCode::OK, Json(response)).into_response()
            }
            Err(error) => upstream_error(error.to_string()).into_response(),
        },
        Err(error) => upstream_error(error.to_string()).into_response(),
    }
}

async fn anthropic_messages(
    State(state): State<Arc<ApiState>>,
    Json(request): Json<Value>,
) -> Response {
    let model = request
        .get("model")
        .and_then(Value::as_str)
        .unwrap_or(&state.default_model);
    let prompt = request
        .get("messages")
        .and_then(Value::as_array)
        .map(|messages| {
            messages
                .iter()
                .filter_map(|message| {
                    Some(format!(
                        "{}: {}",
                        message.get("role")?.as_str()?,
                        message.get("content")?.as_str()?
                    ))
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();
    let stream = request
        .get("stream")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let ollama =
        json!({"model":model,"messages":[{"role":"user","content":prompt}],"stream":stream});
    let response = state
        .client
        .post(format!("{}/api/chat", state.upstream))
        .json(&ollama)
        .send()
        .await;
    match response {
        Ok(response) if stream => anthropic_stream(response, model.to_owned()).into_response(),
        Ok(response) => match response.json::<Value>().await {
            Ok(value) => {
                let content = value
                    .pointer("/message/content")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                (StatusCode::OK, Json(json!({
                        "id":"msg_sanctum_local",
                        "type":"message",
                        "role":"assistant",
                        "model":model,
                        "content":[{"type":"text","text":content}],
                        "stop_reason":"end_turn",
                        "stop_sequence":null,
                        "usage":{"input_tokens":value.get("prompt_eval_count").and_then(Value::as_u64).unwrap_or(0),"output_tokens":value.get("eval_count").and_then(Value::as_u64).unwrap_or(0)}
                    }))).into_response()
            }
            Err(error) => upstream_error(error.to_string()).into_response(),
        },
        Err(error) => upstream_error(error.to_string()).into_response(),
    }
}

fn openai_stream(response: reqwest::Response, model: String) -> Response {
    let mut input = response.bytes_stream();
    let stream = async_stream::stream! {
        let mut buffer = Vec::new();
        while let Some(chunk) = input.next().await {
            match chunk {
                Ok(chunk) => {
                    buffer.extend_from_slice(&chunk);
                    while let Some(index) = buffer.iter().position(|byte| *byte == b'\n') {
                        let line: Vec<_> = buffer.drain(..=index).collect();
                        let line = line.iter().copied().filter(|byte| *byte != b'\n' && *byte != b'\r').collect::<Vec<_>>();
                        if line.is_empty() { continue; }
                        if let Ok(value) = serde_json::from_slice::<Value>(&line) {
                            let content = value.pointer("/message/content").and_then(Value::as_str).unwrap_or_default();
                            let done = value.get("done").and_then(Value::as_bool).unwrap_or(false);
                            let event = json!({
                                "id":"chatcmpl-sanctum-local",
                                "object":"chat.completion.chunk",
                                "model":model,
                                "choices":[{"index":0,"delta":{"content":content},"finish_reason":if done { Some("stop") } else { None::<&str> }}]
                            });
                            yield bytes::Bytes::from(format!("data: {event}\n\n"));
                            if done { yield bytes::Bytes::from_static(b"data: [DONE]\n\n"); }
                        }
                    }
                }
                Err(error) => {
                    let event = json!({"error":{"message":error.to_string(),"type":"upstream_error"}});
                    yield bytes::Bytes::from(format!("data: {event}\n\n"));
                    break;
                }
            }
        }
    };
    let stream = stream.map(Ok::<_, std::convert::Infallible>);
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/event-stream")
        .body(Body::from_stream(stream))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}

fn anthropic_stream(response: reqwest::Response, model: String) -> Response {
    let mut input = response.bytes_stream();
    let stream = async_stream::stream! {
        let mut buffer = Vec::new();
        yield bytes::Bytes::from(format!("event: message_start\ndata: {}\n\n", json!({"type":"message_start","message":{"id":"msg_sanctum_local","type":"message","role":"assistant","model":model,"content":[],"stop_reason":null,"stop_sequence":null,"usage":{"input_tokens":0,"output_tokens":0}}})));
        while let Some(chunk) = input.next().await {
            match chunk {
                Ok(chunk) => {
                    buffer.extend_from_slice(&chunk);
                    while let Some(index) = buffer.iter().position(|byte| *byte == b'\n') {
                        let line: Vec<_> = buffer.drain(..=index).collect();
                        let line = line.iter().copied().filter(|byte| *byte != b'\n' && *byte != b'\r').collect::<Vec<_>>();
                        if line.is_empty() { continue; }
                        if let Ok(value) = serde_json::from_slice::<Value>(&line) {
                            let content = value.pointer("/message/content").and_then(Value::as_str).unwrap_or_default();
                            if !content.is_empty() {
                                yield bytes::Bytes::from(format!("event: content_block_delta\ndata: {}\n\n", json!({"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":content}})));
                            }
                            if value.get("done").and_then(Value::as_bool).unwrap_or(false) {
                                yield bytes::Bytes::from(format!("event: message_delta\ndata: {}\n\n", json!({"type":"message_delta","delta":{"stop_reason":"end_turn","stop_sequence":null},"usage":{"output_tokens":0}})));
                                yield bytes::Bytes::from_static(b"event: message_stop\ndata: {\"type\":\"message_stop\"}\n\n");
                            }
                        }
                    }
                }
                Err(error) => {
                    yield bytes::Bytes::from(format!("event: error\ndata: {}\n\n", json!({"type":"error","error":{"type":"upstream_error","message":error.to_string()}})));
                    break;
                }
            }
        }
    };
    let stream = stream.map(Ok::<_, std::convert::Infallible>);
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/event-stream")
        .body(Body::from_stream(stream))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}

fn openai_prompt(request: &Value) -> String {
    if let Some(input) = request.get("input").and_then(Value::as_str) {
        return input.to_owned();
    }
    request
        .get("messages")
        .and_then(Value::as_array)
        .map(|messages| {
            messages
                .iter()
                .filter_map(|message| {
                    Some(format!(
                        "{}: {}",
                        message.get("role")?.as_str()?,
                        message.get("content")?.as_str()?
                    ))
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default()
}

fn upstream_error(message: impl Into<String>) -> (StatusCode, Json<Value>) {
    let message = message.into();
    (
        StatusCode::BAD_GATEWAY,
        Json(json!({"error":{"message":message,"type":"upstream_error"}})),
    )
}
