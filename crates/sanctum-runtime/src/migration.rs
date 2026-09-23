//! Thunderbolt storage migration and rollback.
#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::unnested_or_patterns
)]

#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationError {
    InvalidInput(&'static str),
    ToolFailure(String),
    Io(String),
}
impl std::fmt::Display for MigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for MigrationError {}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinkWidth(u8);
impl LinkWidth {
    pub fn new(lanes: u8) -> Result<Self, MigrationError> {
        if lanes == 0 || lanes > 16 {
            return Err(MigrationError::InvalidInput("link width"));
        }
        Ok(Self(lanes))
    }
    pub const fn lanes(self) -> u8 {
        self.0
    }
}
impl LinkWidth {
    pub fn parse(value: &str) -> Result<Self, MigrationError> {
        let lanes = value
            .trim()
            .trim_start_matches('x')
            .parse::<u8>()
            .map_err(|_| MigrationError::InvalidInput("link width"))?;
        Self::new(lanes)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkSpeed {
    Gt8,
    Gt16,
}
impl LinkSpeed {
    pub const fn gt_per_second(self) -> u8 {
        match self {
            Self::Gt8 => 8,
            Self::Gt16 => 16,
        }
    }
}
impl LinkSpeed {
    pub fn parse(value: &str) -> Result<Self, MigrationError> {
        match value.trim() {
            "8.0 GT/s" => Ok(Self::Gt8),
            "16.0 GT/s" => Ok(Self::Gt16),
            _ => Err(MigrationError::InvalidInput("link speed")),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortStatus {
    Connected,
    Disconnected,
}
impl PortStatus {
    pub fn parse(value: &str) -> Self {
        let value = value.trim().to_ascii_lowercase();
        if value.contains("connected") && !value.contains("no device") {
            Self::Connected
        } else {
            Self::Disconnected
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BusInspection {
    pub width: LinkWidth,
    pub speed: LinkSpeed,
}
impl BusInspection {
    pub const fn qualifies(self) -> bool {
        self.width.lanes() == 4
    }
}
impl BusInspection {
    pub fn parse(report: &str) -> Result<Self, MigrationError> {
        let mut port = false;
        let mut connected = false;
        let mut width = None;
        let mut speed = None;
        for line in report.lines() {
            let line = line.trim();
            if line == "Port:" {
                if connected {
                    if let (Some(width), Some(speed)) = (width, speed) {
                        return Ok(Self { width, speed });
                    }
                }
                port = true;
                connected = false;
                width = None;
                speed = None;
                continue;
            }
            if !port {
                continue;
            }
            if let Some(value) = line.strip_prefix("Status:") {
                connected = PortStatus::parse(value) == PortStatus::Connected;
            }
            if let Some(value) = line.strip_prefix("Link Width:") {
                width = LinkWidth::parse(value).ok();
            }
            if let Some(value) = line.strip_prefix("Link Speed:") {
                speed = LinkSpeed::parse(value).ok();
            }
        }
        if connected {
            if let (Some(width), Some(speed)) = (width, speed) {
                return Ok(Self { width, speed });
            }
        }
        Err(MigrationError::InvalidInput(
            "connected Thunderbolt link with PCIe width and speed",
        ))
    }
}
pub fn profiler_arguments() -> [&'static str; 1] {
    ["SPThunderboltDataType"]
}
pub fn inspect_live_bus() -> Result<BusInspection, MigrationError> {
    let output = std::process::Command::new("system_profiler")
        .args(profiler_arguments())
        .output()
        .map_err(|error| MigrationError::Io(error.to_string()))?;
    if !output.status.success() {
        return Err(MigrationError::ToolFailure("system_profiler".into()));
    }
    BusInspection::parse(&String::from_utf8_lossy(&output.stdout))
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrimStatus {
    Observed,
    Absent,
    Unsupported,
}
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct TargetVolumeIdentity{pub volume_uuid:String,pub device_identifier:String}
impl TargetVolumeIdentity{pub fn parse_plist(bytes:&[u8])->Result<Self,MigrationError>{let value=plist::Value::from_reader(std::io::Cursor::new(bytes)).map_err(|error|MigrationError::Io(error.to_string()))?;let dictionary=value.as_dictionary().ok_or(MigrationError::InvalidInput("diskutil dictionary"))?;let get=|name:&str|dictionary.get(name).and_then(plist::Value::as_string).ok_or(MigrationError::InvalidInput("diskutil volume identity"));if get("FilesystemType")?!="apfs"{return Err(MigrationError::InvalidInput("destination APFS filesystem"));}let volume_uuid=get("VolumeUUID")?.to_ascii_uppercase();let device_identifier=get("DeviceIdentifier")?.to_owned();if volume_uuid.len()!=36||!volume_uuid.bytes().all(|byte|byte.is_ascii_hexdigit()||byte==b'-')||!device_identifier.starts_with("disk")||!device_identifier.bytes().all(|byte|byte.is_ascii_alphanumeric()){return Err(MigrationError::InvalidInput("diskutil volume identity"));}Ok(Self{volume_uuid,device_identifier})}}
pub fn parse_trim_log_for_volume(log:&str,identity:&TargetVolumeIdentity)->TrimStatus{let relevant=log.lines().filter(|line|line.split(|character:char|!(character.is_ascii_alphanumeric()||character=='-')).any(|token|token.eq_ignore_ascii_case(&identity.volume_uuid)||token.eq_ignore_ascii_case(&identity.device_identifier))).collect::<Vec<_>>().join("\n");parse_trim_log(&relevant)}
pub fn inspect_target_volume(path:&std::path::Path)->Result<TargetVolumeIdentity,MigrationError>{let output=std::process::Command::new("diskutil").arg("info").arg("-plist").arg(path).output().map_err(|error|MigrationError::Io(error.to_string()))?;if !output.status.success(){return Err(MigrationError::ToolFailure("diskutil info".into()));}TargetVolumeIdentity::parse_plist(&output.stdout)}
pub fn inspect_live_trim_for_target(path:&std::path::Path)->Result<TrimEvidence,MigrationError>{let identity=inspect_target_volume(path)?;let output=std::process::Command::new("log").args(trim_log_arguments()).arg(TRIM_LOG_PREDICATE).output().map_err(|error|MigrationError::Io(error.to_string()))?;if !output.status.success(){return Err(MigrationError::ToolFailure("log show".into()));}Ok(TrimEvidence{status:parse_trim_log_for_volume(&String::from_utf8_lossy(&output.stdout),&identity),observed_at:std::time::SystemTime::now()})}
pub fn parse_trim_log(log: &str) -> TrimStatus {
    let mut observed = false;
    for line in log.lines() {
        let lower = line.to_ascii_lowercase();
        if !lower.contains("spaceman") || !lower.contains("trim") {
            continue;
        }
        if lower.contains("unsupported") || lower.contains("disabled") {
            return TrimStatus::Unsupported;
        }
        if lower.contains("completed") || lower.contains("issued") || lower.contains("enabled") {
            observed = true;
        }
    }
    if observed {
        TrimStatus::Observed
    } else {
        TrimStatus::Absent
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrimEvidence {
    pub status: TrimStatus,
    pub observed_at: std::time::SystemTime,
}
impl TrimEvidence {
    pub fn is_fresh(self, now: std::time::SystemTime) -> bool {
        now.duration_since(self.observed_at)
            .is_ok_and(|age| age <= std::time::Duration::from_secs(3600))
    }
}
pub fn trim_log_arguments() -> [&'static str; 6] {
    ["show", "--last", "1h", "--style", "compact", "--predicate"]
}
pub const TRIM_LOG_PREDICATE: &str =
    "process == \"kernel\" AND eventMessage CONTAINS[c] \"spaceman\"";
pub fn inspect_live_trim() -> Result<TrimEvidence, MigrationError> {
    let output = std::process::Command::new("log")
        .args(trim_log_arguments())
        .arg(TRIM_LOG_PREDICATE)
        .output()
        .map_err(|error| MigrationError::Io(error.to_string()))?;
    if !output.status.success() {
        return Err(MigrationError::ToolFailure("log show".into()));
    }
    Ok(TrimEvidence {
        status: parse_trim_log(&String::from_utf8_lossy(&output.stdout)),
        observed_at: std::time::SystemTime::now(),
    })
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MigrationPaths {
    pub origin: std::path::PathBuf,
    pub target: std::path::PathBuf,
}
impl MigrationPaths {
    pub fn new(
        origin: std::path::PathBuf,
        target: std::path::PathBuf,
    ) -> Result<Self, MigrationError> {
        use std::path::Component;
        let valid = |path: &std::path::Path| {
            path.is_absolute()
                && path != std::path::Path::new("/")
                && !path
                    .components()
                    .any(|part| matches!(part, Component::ParentDir | Component::CurDir))
        };
        if !valid(&origin)
            || !valid(&target)
            || origin == target
            || origin.starts_with(&target)
            || target.starts_with(&origin)
        {
            return Err(MigrationError::InvalidInput("migration roots"));
        }
        Ok(Self { origin, target })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncPass {
    Initial,
    Final,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsyncInvocation {
    pub executable: std::path::PathBuf,
    pub arguments: Vec<std::ffi::OsString>,
    pub pass: SyncPass,
}
impl RsyncInvocation {
    pub fn new(executable: std::path::PathBuf, paths: &MigrationPaths, pass: SyncPass) -> Self {
        let mut source = paths.origin.as_os_str().to_os_string();
        source.push("/");
        let arguments = vec![
            "-avXHE".into(),
            "--".into(),
            source,
            paths.target.as_os_str().to_os_string(),
        ];
        Self {
            executable,
            arguments,
            pass,
        }
    }
}
impl RsyncInvocation {
    pub fn run(&self) -> Result<(), MigrationError> {
        let help = std::process::Command::new(&self.executable)
            .arg("--help")
            .output()
            .map_err(|error| MigrationError::Io(error.to_string()))?;
        if !help.status.success()
            || !supports_required_rsync_flags(&String::from_utf8_lossy(&help.stdout))
        {
            return Err(MigrationError::InvalidInput("rsync -avXHE support"));
        }
        let status = std::process::Command::new(&self.executable)
            .args(&self.arguments)
            .status()
            .map_err(|error| MigrationError::Io(error.to_string()))?;
        if !status.success() {
            return Err(MigrationError::ToolFailure(format!(
                "rsync pass {:?}",
                self.pass
            )));
        }
        Ok(())
    }
}
pub fn supports_required_rsync_flags(help: &str) -> bool {
    let short = help
        .lines()
        .find(|line| line.to_ascii_lowercase().contains("usage: rsync ["))
        .unwrap_or("");
    let long = help.contains("-X, --xattrs")
        && help.contains("-H, --hard-links")
        && help.contains("-E, --executability");
    (short.contains('X') && short.contains('H') && short.contains('E')) || long
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sha256Digest(pub [u8; 32]);
impl Sha256Digest {
    pub fn to_hex(self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut result = String::with_capacity(64);
        for byte in self.0 {
            result.push(char::from(HEX[usize::from(byte >> 4)]));
            result.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
        result
    }
}
pub fn hash_file(path: &std::path::Path) -> Result<Sha256Digest, MigrationError> {
    use sha2::{Digest, Sha256};
    use std::io::Read;
    let mut file =
        std::fs::File::open(path).map_err(|error| MigrationError::Io(error.to_string()))?;
    let mut hash = Sha256::new();
    let mut chunk = [0_u8; 16 * 1024];
    loop {
        let size = file
            .read(&mut chunk)
            .map_err(|error| MigrationError::Io(error.to_string()))?;
        if size == 0 {
            break;
        }
        hash.update(&chunk[..size]);
    }
    Ok(Sha256Digest(hash.finalize().into()))
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestEntry {
    Directory,
    File { bytes: u64, digest: Sha256Digest },
    Symlink(std::path::PathBuf),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationManifest {
    pub entries: std::collections::BTreeMap<std::path::PathBuf, ManifestEntry>,
}
impl MigrationManifest {
    pub fn scan(root: &std::path::Path) -> Result<Self, MigrationError> {
        let metadata = std::fs::symlink_metadata(root)
            .map_err(|error| MigrationError::Io(error.to_string()))?;
        if !metadata.is_dir() {
            return Err(MigrationError::InvalidInput("manifest root"));
        }
        let mut entries = std::collections::BTreeMap::new();
        Self::visit(root, root, &mut entries)?;
        Ok(Self { entries })
    }
}
impl MigrationManifest {
    fn visit(
        root: &std::path::Path,
        path: &std::path::Path,
        entries: &mut std::collections::BTreeMap<std::path::PathBuf, ManifestEntry>,
    ) -> Result<(), MigrationError> {
        let mut children = std::fs::read_dir(path)
            .map_err(|error| MigrationError::Io(error.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| MigrationError::Io(error.to_string()))?;
        children.sort_by_key(std::fs::DirEntry::file_name);
        for child in children {
            let path = child.path();
            let relative = path
                .strip_prefix(root)
                .map_err(|_| MigrationError::InvalidInput("manifest path"))?
                .to_path_buf();
            let metadata = std::fs::symlink_metadata(&path)
                .map_err(|error| MigrationError::Io(error.to_string()))?;
            let entry = if metadata.file_type().is_symlink() {
                ManifestEntry::Symlink(
                    std::fs::read_link(&path)
                        .map_err(|error| MigrationError::Io(error.to_string()))?,
                )
            } else if metadata.is_dir() {
                ManifestEntry::Directory
            } else if metadata.is_file() {
                ManifestEntry::File {
                    bytes: metadata.len(),
                    digest: hash_file(&path)?,
                }
            } else {
                return Err(MigrationError::InvalidInput("special file"));
            };
            let recurse = matches!(entry, ManifestEntry::Directory);
            entries.insert(relative, entry);
            if recurse {
                Self::visit(root, &path, entries)?;
            }
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestDifference {
    Missing(std::path::PathBuf),
    Extra(std::path::PathBuf),
    Changed(std::path::PathBuf),
}
pub fn compare_manifests(
    source: &MigrationManifest,
    target: &MigrationManifest,
) -> Result<(), ManifestDifference> {
    for (path, expected) in &source.entries {
        match target.entries.get(path) {
            None => return Err(ManifestDifference::Missing(path.clone())),
            Some(actual) if actual != expected => {
                return Err(ManifestDifference::Changed(path.clone()));
            }
            Some(_) => {}
        }
    }
    for path in target.entries.keys() {
        if !source.entries.contains_key(path) {
            return Err(ManifestDifference::Extra(path.clone()));
        }
    }
    Ok(())
}
pub fn verify_roots(paths: &MigrationPaths) -> Result<(), MigrationError> {
    let source = MigrationManifest::scan(&paths.origin)?;
    let target = MigrationManifest::scan(&paths.target)?;
    compare_manifests(&source, &target).map_err(|difference| {
        MigrationError::InvalidInput(match difference {
            ManifestDifference::Missing(_) => "missing target entry",
            ManifestDifference::Extra(_) => "unexpected target entry",
            ManifestDifference::Changed(_) => "changed target entry",
        })
    })
}
pub fn confirm_quiescence(input: &str) -> Result<(), MigrationError> {
    if input.trim_end_matches(['\r', '\n']) == "QUIESCED" {
        Ok(())
    } else {
        Err(MigrationError::InvalidInput("quiescence confirmation"))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MigrationStage {
    Prepared,
    InitialSynced,
    Verified,
    Activated,
    RolledBack,
    Completed,
}
impl MigrationStage {
    pub fn transition(&mut self, next: Self) -> Result<(), MigrationError> {
        let valid = matches!(
            (*self, next),
            (Self::Prepared, Self::InitialSynced)
                | (Self::InitialSynced, Self::Verified)
                | (Self::Verified, Self::Activated)
                | (Self::Activated, Self::RolledBack)
                | (Self::Activated, Self::Completed)
        );
        if !valid {
            return Err(MigrationError::InvalidInput("migration stage"));
        }
        *self = next;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CutoverRecord {
    pub paths: MigrationPaths,
    pub stage: MigrationStage,
    pub activated_at: u64,
}
impl CutoverRecord {
    pub fn encode(&self) -> Result<Vec<u8>, MigrationError> {
        serde_json::to_vec_pretty(self).map_err(|error| MigrationError::Io(error.to_string()))
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, MigrationError> {
        serde_json::from_slice(bytes).map_err(|error| {
            MigrationError::InvalidInput(if error.is_syntax() {
                "cutover record syntax"
            } else {
                "cutover record"
            })
        })
    }
}
pub const ROLLBACK_WINDOW_SECONDS: u64 = 72 * 60 * 60;
impl CutoverRecord {
    pub fn rollback_deadline(&self) -> Result<u64, MigrationError> {
        self.activated_at
            .checked_add(ROLLBACK_WINDOW_SECONDS)
            .ok_or(MigrationError::InvalidInput("rollback deadline"))
    }
    pub fn rollback_active(&self, now: u64) -> bool {
        self.stage == MigrationStage::Activated
            && self
                .rollback_deadline()
                .is_ok_and(|deadline| now <= deadline && now >= self.activated_at)
    }
}
pub fn write_atomic(path: &std::path::Path, bytes: &[u8]) -> Result<(), MigrationError> {
    use std::io::Write;
    let parent = path
        .parent()
        .ok_or(MigrationError::InvalidInput("state path"))?;
    std::fs::create_dir_all(parent).map_err(|error| MigrationError::Io(error.to_string()))?;
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| MigrationError::InvalidInput("clock"))?
        .as_nanos();
    let temp = path.with_extension(format!("tmp-{}-{nonce}", std::process::id()));
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)
        .map_err(|error| MigrationError::Io(error.to_string()))?;
    file.write_all(bytes)
        .map_err(|error| MigrationError::Io(error.to_string()))?;
    file.sync_all()
        .map_err(|error| MigrationError::Io(error.to_string()))?;
    std::fs::rename(&temp, path).map_err(|error| MigrationError::Io(error.to_string()))?;
    std::fs::File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| MigrationError::Io(error.to_string()))
}
pub fn write_ai_root(
    pointer: &std::path::Path,
    root: &std::path::Path,
) -> Result<(), MigrationError> {
    if !root.is_absolute() || root.as_os_str().to_string_lossy().contains('\n') {
        return Err(MigrationError::InvalidInput("AI_ROOT path"));
    }
    let mut line = root.as_os_str().to_string_lossy().as_bytes().to_vec();
    line.push(b'\n');
    write_atomic(pointer, &line)
}
pub fn save_record(path: &std::path::Path, record: &CutoverRecord) -> Result<(), MigrationError> {
    write_atomic(path, &record.encode()?)
}
pub fn load_record(path: &std::path::Path) -> Result<CutoverRecord, MigrationError> {
    CutoverRecord::decode(
        &std::fs::read(path).map_err(|error| MigrationError::Io(error.to_string()))?,
    )
}
pub fn rollback(
    record_path: &std::path::Path,
    pointer: &std::path::Path,
) -> Result<CutoverRecord, MigrationError> {
    let mut record = load_record(record_path)?;
    if record.stage != MigrationStage::Activated {
        return Err(MigrationError::InvalidInput("rollback stage"));
    }
    if !record.paths.origin.is_dir() {
        return Err(MigrationError::InvalidInput("origin unavailable"));
    }
    write_ai_root(pointer, &record.paths.origin)?;
    record.stage.transition(MigrationStage::RolledBack)?;
    save_record(record_path, &record)?;
    Ok(record)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailoverCause {
    LinkDropped,
    KernelPanic,
    TargetUnavailable,
}
pub fn assess_failover(
    report: &str,
    panic_log: &str,
    target_available: bool,
) -> Option<FailoverCause> {
    if !target_available {
        return Some(FailoverCause::TargetUnavailable);
    }
    if BusInspection::parse(report).map_or(true, |link| !link.qualifies()) {
        return Some(FailoverCause::LinkDropped);
    }
    if panic_log.lines().any(|line| {
        let lower = line.to_ascii_lowercase();
        lower.contains("kernel") && lower.contains("panic")
    }) {
        return Some(FailoverCause::KernelPanic);
    }
    None
}
pub fn monitor_once(
    record_path: &std::path::Path,
    pointer: &std::path::Path,
) -> Result<Option<FailoverCause>, MigrationError> {
    let mut record = load_record(record_path)?;
    if record.stage != MigrationStage::Activated {
        return Ok(None);
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| MigrationError::InvalidInput("clock"))?
        .as_secs();
    if !record.rollback_active(now) {
        record.stage.transition(MigrationStage::Completed)?;
        save_record(record_path, &record)?;
        return Ok(None);
    }
    let bus = std::process::Command::new("system_profiler")
        .args(profiler_arguments())
        .output()
        .map_err(|error| MigrationError::Io(error.to_string()))?;
    let panic = std::process::Command::new("log")
        .args([
            "show",
            "--last",
            "5m",
            "--style",
            "compact",
            "--predicate",
            "process == \"kernel\" AND eventMessage CONTAINS[c] \"panic\"",
        ])
        .output()
        .map_err(|error| MigrationError::Io(error.to_string()))?;
    if !panic.status.success() {
        return Err(MigrationError::ToolFailure("panic log".into()));
    }
    let bus_report = if bus.status.success() {
        String::from_utf8_lossy(&bus.stdout)
    } else {
        std::borrow::Cow::Borrowed("")
    };
    let cause = assess_failover(
        &bus_report,
        &String::from_utf8_lossy(&panic.stdout),
        record.paths.target.is_dir(),
    );
    if cause.is_some() {
        rollback(record_path, pointer)?;
    }
    Ok(cause)
}
pub fn execute_migration(
    paths: MigrationPaths,
    rsync: &std::path::Path,
    record_path: &std::path::Path,
    pointer: &std::path::Path,
    monitor_executable: &std::path::Path,
) -> Result<CutoverRecord, MigrationError> {
    if !paths.origin.is_dir() || !paths.target.is_dir() {
        return Err(MigrationError::InvalidInput("mounted migration roots"));
    }
    if std::fs::read_dir(&paths.target)
        .map_err(|error| MigrationError::Io(error.to_string()))?
        .next()
        .is_some()
    {
        return Err(MigrationError::InvalidInput("target must be empty"));
    }
    if !inspect_live_bus()?.qualifies() {
        return Err(MigrationError::InvalidInput("x4 Thunderbolt link"));
    }
    let trim = inspect_live_trim_for_target(&paths.target)?;
    if trim.status != TrimStatus::Observed || !trim.is_fresh(std::time::SystemTime::now()) {
        return Err(MigrationError::InvalidInput("fresh APFS TRIM evidence"));
    }
    let mut record = CutoverRecord {
        paths,
        stage: MigrationStage::Prepared,
        activated_at: 0,
    };
    save_record(record_path, &record)?;
    RsyncInvocation::new(rsync.to_path_buf(), &record.paths, SyncPass::Initial).run()?;
    record.stage.transition(MigrationStage::InitialSynced)?;
    save_record(record_path, &record)?;
    RsyncInvocation::new(rsync.to_path_buf(), &record.paths, SyncPass::Final).run()?;
    verify_roots(&record.paths)?;
    record.stage.transition(MigrationStage::Verified)?;
    save_record(record_path, &record)?;
    record.activated_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| MigrationError::InvalidInput("clock"))?
        .as_secs();
    record.stage.transition(MigrationStage::Activated)?;
    save_record(record_path, &record)?;
    write_ai_root(pointer, &record.paths.target)?;
    let mut command = Command::new(monitor_executable);
    command
        .arg("monitor")
        .arg(record_path)
        .arg(pointer)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(unix)]
    {
        command.process_group(0);
    }
    if let Err(error) = command.spawn() {
        rollback(record_path, pointer)?;
        return Err(MigrationError::Io(error.to_string()));
    }
    Ok(record)
}
// Migration extensions.
