//! Asynchronous model-process supervision.
#![allow(clippy::missing_errors_doc)]

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SupervisorError {
    #[error("subprocess identifier is unavailable")]
    MissingProcessId,
    #[error("{field} path must be absolute: {path}")]
    RelativePath {
        field: &'static str,
        path: std::path::PathBuf,
    },
    #[error("invalid subprocess environment {field}")]
    InvalidEnvironment { field: &'static str },
    #[error("{field} timeout must be nonzero")]
    ZeroTimeout { field: &'static str },
    #[error("subprocess {operation} failed: {source}")]
    ProcessIo {
        operation: &'static str,
        #[source]
        source: std::io::Error,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvironmentEntry {
    key: String,
    value: String,
}

impl EnvironmentEntry {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Result<Self, SupervisorError> {
        let key = key.into();
        let value = value.into();
        if key.contains('=') || key.contains('\0') {
            return Err(SupervisorError::InvalidEnvironment { field: "key" });
        }
        if value.contains('\0') {
            return Err(SupervisorError::InvalidEnvironment { field: "value" });
        }
        Ok(Self { key, value })
    }
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutablePath(std::path::PathBuf);

impl ExecutablePath {
    pub fn new(path: impl Into<std::path::PathBuf>) -> Result<Self, SupervisorError> {
        let path = path.into();
        if !path.is_absolute() {
            return Err(SupervisorError::RelativePath {
                field: "executable",
                path,
            });
        }
        Ok(Self(path))
    }

    #[must_use]
    pub fn as_path(&self) -> &std::path::Path {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelPath(std::path::PathBuf);

impl ModelPath {
    pub fn new(path: impl Into<std::path::PathBuf>) -> Result<Self, SupervisorError> {
        let path = path.into();
        if !path.is_absolute() {
            return Err(SupervisorError::RelativePath {
                field: "model",
                path,
            });
        }
        Ok(Self(path))
    }

    #[must_use]
    pub fn as_path(&self) -> &std::path::Path {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessSpec {
    pub backend: BackendKind,
    pub executable: ExecutablePath,
    pub model: ModelPath,
    pub arguments: Vec<String>,
    pub environment: Vec<EnvironmentEntry>,
    pub working_directory: Option<std::path::PathBuf>,
}

impl ProcessSpec {
    #[must_use]
    pub fn with_working_directory(mut self, path: std::path::PathBuf) -> Self {
        self.working_directory = Some(path);
        self
    }

    #[must_use]
    pub fn with_environment(mut self, entry: EnvironmentEntry) -> Self {
        self.environment.push(entry);
        self
    }

    #[must_use]
    pub fn with_argument(mut self, argument: impl Into<String>) -> Self {
        self.arguments.push(argument.into());
        self
    }

    #[must_use]
    pub fn llama_server(executable: ExecutablePath, model: ModelPath) -> Self {
        Self {
            backend: BackendKind::LlamaServer,
            executable,
            model,
            arguments: Vec::new(),
            environment: Vec::new(),
            working_directory: None,
        }
    }

    #[must_use]
    pub fn mlx_lm(executable: ExecutablePath, model: ModelPath) -> Self {
        Self {
            backend: BackendKind::MlxLm,
            executable,
            model,
            arguments: Vec::new(),
            environment: Vec::new(),
            working_directory: None,
        }
    }
}

#[must_use]
pub fn build_command(spec: &ProcessSpec) -> tokio::process::Command {
    let mut command = tokio::process::Command::new(spec.executable.as_path());
    command.args(&spec.arguments);
    command.envs(
        spec.environment
            .iter()
            .map(|entry| (entry.key(), entry.value())),
    );
    if let Some(directory) = &spec.working_directory {
        command.current_dir(directory);
    }
    command.stdin(std::process::Stdio::null());
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());
    command
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackendKind {
    LlamaServer,
    MlxLm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Starting,
    Running,
    Stopping,
    Exited,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessSignal {
    Terminate,
    Kill,
}

impl ProcessSignal {
    #[must_use]
    pub const fn number(self) -> libc::c_int {
        match self {
            Self::Terminate => libc::SIGTERM,
            Self::Kill => libc::SIGKILL,
        }
    }
}

fn send_signal(pid: u32, signal: ProcessSignal) -> Result<(), SupervisorError> {
    let pid = libc::pid_t::try_from(pid).map_err(|_| SupervisorError::MissingProcessId)?;
    let result = unsafe { libc::kill(pid, signal.number()) };
    if result == 0 {
        return Ok(());
    }
    Err(SupervisorError::ProcessIo {
        operation: "signal",
        source: std::io::Error::last_os_error(),
    })
}

#[derive(Debug)]
pub struct SupervisedChild {
    child: tokio::process::Child,
    backend: BackendKind,
    state: ProcessState,
}

impl SupervisedChild {
    pub fn terminate(&mut self) -> Result<(), SupervisorError> {
        self.signal(ProcessSignal::Terminate)
    }

    pub fn force_kill(&mut self) -> Result<(), SupervisorError> {
        self.signal(ProcessSignal::Kill)
    }

    pub async fn shutdown(
        &mut self,
        policy: ShutdownPolicy,
    ) -> Result<ShutdownOutcome, SupervisorError> {
        if self.try_status()?.is_some() {
            return Ok(ShutdownOutcome::AlreadyExited);
        }
        self.terminate()?;
        if let Ok(result) = tokio::time::timeout(policy.graceful, self.wait()).await {
            result?;
            Ok(ShutdownOutcome::Graceful)
        } else {
            self.force_kill()?;
            self.wait().await?;
            Ok(ShutdownOutcome::Forced)
        }
    }

    pub fn signal(&mut self, signal: ProcessSignal) -> Result<(), SupervisorError> {
        let pid = self.id().ok_or(SupervisorError::MissingProcessId)?;
        send_signal(pid, signal)?;
        self.state = ProcessState::Stopping;
        Ok(())
    }

    pub fn spawn(spec: &ProcessSpec) -> Result<Self, SupervisorError> {
        let child = build_command(spec)
            .spawn()
            .map_err(|source| SupervisorError::ProcessIo {
                operation: "spawn",
                source,
            })?;
        Ok(Self {
            child,
            backend: spec.backend,
            state: ProcessState::Running,
        })
    }

    #[must_use]
    pub fn id(&self) -> Option<u32> {
        self.child.id()
    }

    #[must_use]
    pub const fn state(&self) -> ProcessState {
        self.state
    }

    #[must_use]
    pub const fn backend(&self) -> BackendKind {
        self.backend
    }

    pub fn try_status(&mut self) -> Result<Option<std::process::ExitStatus>, SupervisorError> {
        let status = self
            .child
            .try_wait()
            .map_err(|source| SupervisorError::ProcessIo {
                operation: "query status",
                source,
            })?;
        if status.is_some() {
            self.state = ProcessState::Exited;
        }
        Ok(status)
    }

    pub async fn wait(&mut self) -> Result<std::process::ExitStatus, SupervisorError> {
        let status = self
            .child
            .wait()
            .await
            .map_err(|source| SupervisorError::ProcessIo {
                operation: "wait",
                source,
            })?;
        self.state = ProcessState::Exited;
        Ok(status)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShutdownPolicy {
    pub graceful: std::time::Duration,
    pub serialization: std::time::Duration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheMarker(std::path::PathBuf);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShutdownOutcome {
    Graceful,
    Forced,
    AlreadyExited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupervisorEvent {
    Spawned { backend: BackendKind, pid: u32 },
    TerminationRequested { pid: u32 },
    Exited { pid: u32, forced: bool },
}

impl CacheMarker {
    pub fn new(path: impl Into<std::path::PathBuf>) -> Result<Self, SupervisorError> {
        let path = path.into();
        if !path.is_absolute() {
            return Err(SupervisorError::RelativePath {
                field: "cache marker",
                path,
            });
        }
        Ok(Self(path))
    }

    pub async fn is_complete(&self) -> Result<bool, SupervisorError> {
        tokio::fs::try_exists(&self.0)
            .await
            .map_err(|source| SupervisorError::ProcessIo {
                operation: "inspect cache marker",
                source,
            })
    }
}

impl ShutdownPolicy {
    pub fn new(
        graceful: std::time::Duration,
        serialization: std::time::Duration,
    ) -> Result<Self, SupervisorError> {
        if graceful.is_zero() {
            return Err(SupervisorError::ZeroTimeout { field: "graceful" });
        }
        if serialization.is_zero() {
            return Err(SupervisorError::ZeroTimeout {
                field: "serialization",
            });
        }
        Ok(Self {
            graceful,
            serialization,
        })
    }
}

impl ProcessState {
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Exited | Self::Failed)
    }
}

impl std::fmt::Display for BackendKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::LlamaServer => "llama-server",
            Self::MlxLm => "mlx-lm",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BackendKind, EnvironmentEntry, ExecutablePath, ModelPath, ProcessState, ShutdownPolicy,
        SupervisorError,
    };

    #[test]
    fn maps_termination_signal() {
        assert_eq!(super::ProcessSignal::Terminate.number(), libc::SIGTERM);
    }

    #[tokio::test]
    async fn reports_missing_cache_marker() {
        let directory = tempfile::tempdir().unwrap();
        let marker = super::CacheMarker::new(directory.path().join("missing")).unwrap();
        assert!(!marker.is_complete().await.unwrap());
    }

    #[tokio::test]
    async fn reports_completed_cache_marker() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("complete");
        tokio::fs::write(&path, []).await.unwrap();
        let marker = super::CacheMarker::new(path).unwrap();
        assert!(marker.is_complete().await.unwrap());
    }

    #[tokio::test]
    async fn supervises_successful_process_exit() {
        let spec = super::ProcessSpec::llama_server(
            ExecutablePath::new("/usr/bin/true").unwrap(),
            ModelPath::new("/tmp/model").unwrap(),
        );
        let mut child = super::SupervisedChild::spawn(&spec).unwrap();
        assert!(child.wait().await.unwrap().success());
        assert_eq!(child.state(), ProcessState::Exited);
    }

    #[tokio::test]
    async fn preserves_already_exited_shutdown_outcome() {
        let spec = super::ProcessSpec::llama_server(
            ExecutablePath::new("/usr/bin/true").unwrap(),
            ModelPath::new("/tmp/model").unwrap(),
        );
        let mut child = super::SupervisedChild::spawn(&spec).unwrap();
        child.wait().await.unwrap();
        let policy = ShutdownPolicy::new(
            std::time::Duration::from_millis(50),
            std::time::Duration::from_millis(50),
        )
        .unwrap();
        assert_eq!(
            child.shutdown(policy).await.unwrap(),
            super::ShutdownOutcome::AlreadyExited
        );
    }

    #[test]
    fn formats_llama_server_backend() {
        assert_eq!(BackendKind::LlamaServer.to_string(), "llama-server");
    }

    #[test]
    fn formats_mlx_backend() {
        assert_eq!(BackendKind::MlxLm.to_string(), "mlx-lm");
    }

    #[test]
    fn rejects_relative_executable_path() {
        assert!(matches!(
            ExecutablePath::new("bin/server"),
            Err(SupervisorError::RelativePath {
                field: "executable",
                ..
            })
        ));
    }

    #[test]
    fn accepts_absolute_executable_path() {
        let path = ExecutablePath::new("/usr/bin/true").unwrap();
        assert_eq!(path.as_path(), std::path::Path::new("/usr/bin/true"));
    }

    #[test]
    fn rejects_relative_model_path() {
        assert!(matches!(
            ModelPath::new("models/model.gguf"),
            Err(SupervisorError::RelativePath { field: "model", .. })
        ));
    }

    #[test]
    fn accepts_absolute_model_path() {
        let path = ModelPath::new("/Volumes/models/model.gguf").unwrap();
        assert_eq!(
            path.as_path(),
            std::path::Path::new("/Volumes/models/model.gguf")
        );
    }

    #[test]
    fn constructs_llama_server_specification() {
        let spec = super::ProcessSpec::llama_server(
            ExecutablePath::new("/bin/server").unwrap(),
            ModelPath::new("/models/a.gguf").unwrap(),
        );
        assert_eq!(spec.backend, BackendKind::LlamaServer);
    }

    #[test]
    fn constructs_mlx_runtime_specification() {
        let spec = super::ProcessSpec::mlx_lm(
            ExecutablePath::new("/bin/mlx").unwrap(),
            ModelPath::new("/models/a").unwrap(),
        );
        assert_eq!(spec.backend, BackendKind::MlxLm);
    }
    #[test]
    fn appends_process_argument() {
        let spec = super::ProcessSpec::llama_server(
            ExecutablePath::new("/bin/server").unwrap(),
            ModelPath::new("/models/a.gguf").unwrap(),
        )
        .with_argument("--port");
        assert_eq!(spec.arguments, ["--port"]);
    }
    #[test]
    fn appends_process_environment() {
        let entry = EnvironmentEntry::new("MODE", "local").unwrap();
        let spec = super::ProcessSpec::mlx_lm(
            ExecutablePath::new("/bin/mlx").unwrap(),
            ModelPath::new("/models/a").unwrap(),
        )
        .with_environment(entry);
        assert_eq!(spec.environment[0].key(), "MODE");
    }
    #[test]
    fn configures_process_working_directory() {
        let spec = super::ProcessSpec::mlx_lm(
            ExecutablePath::new("/bin/mlx").unwrap(),
            ModelPath::new("/models/a").unwrap(),
        )
        .with_working_directory("/tmp".into());
        assert_eq!(
            spec.working_directory.as_deref(),
            Some(std::path::Path::new("/tmp"))
        );
    }
    #[test]
    fn rejects_environment_assignment_key() {
        assert!(EnvironmentEntry::new("A=B", "x").is_err());
    }
    #[test]
    fn rejects_nul_environment_key() {
        assert!(EnvironmentEntry::new("A\0B", "x").is_err());
    }
    #[test]
    fn rejects_nul_environment_value() {
        assert!(EnvironmentEntry::new("A", "x\0y").is_err());
    }
    #[test]
    fn accepts_valid_environment_entry() {
        let entry = EnvironmentEntry::new("MODEL_HOME", "/models").unwrap();
        assert_eq!((entry.key(), entry.value()), ("MODEL_HOME", "/models"));
    }
    #[test]
    fn running_state_is_not_terminal() {
        assert!(!ProcessState::Running.is_terminal());
    }
    #[test]
    fn exited_state_is_terminal() {
        assert!(ProcessState::Exited.is_terminal());
    }
    #[test]
    fn rejects_zero_graceful_timeout() {
        assert!(
            ShutdownPolicy::new(std::time::Duration::ZERO, std::time::Duration::from_secs(1))
                .is_err()
        );
    }
    #[test]
    fn rejects_zero_serialization_timeout() {
        assert!(
            ShutdownPolicy::new(std::time::Duration::from_secs(1), std::time::Duration::ZERO)
                .is_err()
        );
    }
    #[test]
    fn accepts_valid_shutdown_policy() {
        assert!(
            ShutdownPolicy::new(
                std::time::Duration::from_secs(5),
                std::time::Duration::from_secs(3)
            )
            .is_ok()
        );
    }
}
